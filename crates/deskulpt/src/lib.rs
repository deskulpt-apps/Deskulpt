#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/packages/deskulpt/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/packages/deskulpt/public/deskulpt.svg"
)]

#[cfg(debug_assertions)]
use std::collections::HashSet;
#[cfg(debug_assertions)]
use std::path::{Path, PathBuf};
#[cfg(debug_assertions)]
use std::sync::mpsc::{self, RecvTimeoutError};
#[cfg(debug_assertions)]
use std::thread;
#[cfg(debug_assertions)]
use std::time::Duration;

#[cfg(debug_assertions)]
use anyhow::Context;
#[cfg(debug_assertions)]
use deskulpt_core::events::RenderWidgetsEvent;
use deskulpt_core::path::PathExt;
use deskulpt_core::states::{
    CanvasImodeStateExt, InitialRenderStateExt, SettingsStateExt, WidgetCatalogStateExt,
};
use deskulpt_core::tray::TrayExt;
use deskulpt_core::window::WindowExt;
#[cfg(debug_assertions)]
use notify::event::{CreateKind, ModifyKind, RemoveKind};
#[cfg(debug_assertions)]
use notify::{
    Event, EventKind, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher,
};
use tauri::image::Image;
use tauri::{generate_context, include_image, Builder};
#[cfg(debug_assertions)]
use tauri::{App, AppHandle, Runtime};

/// Image object for the Deskulpt icon.
const DESKULPT_ICON: Image = include_image!("./icons/icon.png");

/// Entry point for the Deskulpt backend.
pub fn run() {
    Builder::default()
        .setup(move |app| {
            app.init_widgets_dir()?;
            app.init_persist_dir()?;

            app.manage_settings();
            app.manage_initial_render();
            app.manage_widget_catalog();
            app.manage_canvas_imode();

            // Hide the application from the dock on macOS because skipping
            // taskbar is not applicable for macOS
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            app.create_manager()?;
            app.create_canvas()?;
            app.create_tray(DESKULPT_ICON)?;

            #[cfg(debug_assertions)]
            if let Err(error) = setup_widget_hot_reload(app) {
                eprintln!("[hot-reload] Failed to initialize widget watcher: {error:?}");
            }

            Ok(())
        })
        .on_window_event(deskulpt_core::window::on_window_event)
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // Prevent the opener plugin from registering handler for click event
        // so we can register our own that opens non-_blank anchors in new tab
        .plugin(
            tauri_plugin_opener::Builder::new()
                .open_js_links_on_click(false)
                .build(),
        )
        .plugin(deskulpt_core::init())
        .run(generate_context!())
        .expect("Error running the Deskulpt application");
}

#[cfg(debug_assertions)]
fn setup_widget_hot_reload<R: Runtime>(app: &App<R>) -> anyhow::Result<()> {
    let widgets_dir = app
        .widgets_dir()
        .context("Widget directory is not initialized")?
        .to_path_buf();
    let app_handle = app.handle();
    let app_handle_for_thread = app_handle.clone();

    thread::Builder::new()
        .name("deskulpt-widget-hot-reload".into())
        .spawn(move || {
            if let Err(err) = watch_widget_changes(app_handle_for_thread, widgets_dir) {
                eprintln!("[hot-reload] Watcher exited: {err:?}");
            }
        })
        .context("Failed to spawn widget watcher thread")?;

    Ok(())
}

#[cfg(debug_assertions)]
fn watch_widget_changes<R: Runtime>(
    app_handle: AppHandle<R>,
    widgets_dir: PathBuf,
) -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel::<NotifyResult<Event>>();

    let mut _watcher: RecommendedWatcher = notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    })?;

    _watcher
        .watch(&widgets_dir, RecursiveMode::Recursive)
        .with_context(|| {
            format!(
                "Failed to watch widgets directory {}",
                widgets_dir.display()
            )
        })?;

    println!(
        "[hot-reload] Watching widget sources in {}",
        widgets_dir.display()
    );

    let debounce = Duration::from_millis(300);
    let mut pending: HashSet<String> = HashSet::new();

    loop {
        match rx.recv_timeout(debounce) {
            Ok(Ok(event)) => {
                if !is_relevant_event(&event.kind) {
                    continue;
                }
                for id in collect_widget_ids(&event, &widgets_dir) {
                    pending.insert(id);
                }
            },
            Ok(Err(err)) => {
                eprintln!("[hot-reload] Watcher reported an error: {err}");
            },
            Err(RecvTimeoutError::Timeout) => {
                flush_pending(&app_handle, &mut pending);
            },
            Err(RecvTimeoutError::Disconnected) => {
                flush_pending(&app_handle, &mut pending);
                break;
            },
        }
    }

    Ok(())
}

#[cfg(debug_assertions)]
fn collect_widget_ids(event: &Event, widgets_dir: &Path) -> Vec<String> {
    let mut ids = HashSet::new();
    for path in &event.paths {
        if let Some(id) = widget_id_from_path(path, widgets_dir, &event.kind) {
            ids.insert(id);
        }
    }
    ids.into_iter().collect()
}

#[cfg(debug_assertions)]
fn widget_id_from_path(path: &Path, widgets_dir: &Path, kind: &EventKind) -> Option<String> {
    let relative = path.strip_prefix(widgets_dir).ok()?;
    if relative.as_os_str().is_empty() {
        return None;
    }

    let mut components = relative.components();
    let id_component = components.next()?;
    let is_top_level = components.next().is_none();

    let id = id_component.as_os_str().to_string_lossy().to_string();

    let matches_extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| matches_ignore_ascii_case(ext))
        .unwrap_or(false);

    let directory_event = is_top_level
        && matches!(
            kind,
            EventKind::Create(CreateKind::Any | CreateKind::Folder)
                | EventKind::Remove(RemoveKind::Any | RemoveKind::Folder)
                | EventKind::Modify(ModifyKind::Name(_))
        );

    if !matches_extension && !directory_event {
        return None;
    }

    Some(id)
}

#[cfg(debug_assertions)]
fn matches_ignore_ascii_case(ext: &str) -> bool {
    matches!(
        ext.to_ascii_lowercase().as_str(),
        "ts" | "tsx" | "js" | "jsx"
    )
}

#[cfg(debug_assertions)]
fn is_relevant_event(kind: &EventKind) -> bool {
    match kind {
        EventKind::Modify(modify) => matches!(
            modify,
            ModifyKind::Any
                | ModifyKind::Data(_)
                | ModifyKind::Metadata(_)
                | ModifyKind::Name(_)
                | ModifyKind::Other
        ),
        EventKind::Create(create) => matches!(
            create,
            CreateKind::Any | CreateKind::File | CreateKind::Folder
        ),
        EventKind::Remove(remove) => matches!(
            remove,
            RemoveKind::Any | RemoveKind::File | RemoveKind::Folder
        ),
        _ => false,
    }
}

#[cfg(debug_assertions)]
fn flush_pending<R: Runtime>(app_handle: &AppHandle<R>, pending: &mut HashSet<String>) {
    if pending.is_empty() {
        return;
    }

    let mut widget_ids: Vec<String> = pending.drain().collect();
    widget_ids.sort();

    println!(
        "[hot-reload] Triggering hot reload for widgets: {}",
        widget_ids.join(", ")
    );

    let event = RenderWidgetsEvent::new(widget_ids);

    if let Err(err) = app_handle.emit_on_render_ready(event) {
        eprintln!("[hot-reload] Failed to emit RenderWidgetsEvent: {err:?}");
    }
}
