#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

use std::fs;
use std::path::Path;

use deskulpt_core::path::PathExt;
use deskulpt_core::shortcuts::ShortcutsExt;
use deskulpt_core::states::{CanvasImodeStateExt, LoggingStateExt};
use deskulpt_core::tray::TrayExt;
use deskulpt_core::window::WindowExt;
use tauri::{Builder, Manager, generate_context};

/// Entry point for the Deskulpt backend.
pub fn run() {
    Builder::default()
        .setup(move |app| {
            app.init_widgets_dir()?;
            app.init_persist_dir()?;
            app.init_logs_dir()?;
            app.manage_logging()?;
            seed_welcome_widget_if_empty(app)?;

            // Hide the application from the dock on macOS because skipping
            // taskbar is not applicable for macOS
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            app.init_shortcuts();
            app.create_manager()?;
            app.create_canvas()?;
            app.create_tray()?;

            app.manage_canvas_imode()?;

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
        .plugin(deskulpt_settings::init())
        .plugin(deskulpt_widgets::init())
        .run(generate_context!())
        .expect("Error running the Deskulpt application");
}

fn seed_welcome_widget_if_empty<R: tauri::Runtime>(app: &tauri::App<R>) -> tauri::Result<bool> {
    let widgets_dir = app.widgets_dir()?;
    if widgets_dir.read_dir()?.next().is_some() {
        return Ok(false);
    }

    let resource_dir = app.path().resource_dir()?;
    let src = resource_dir.join("default-widgets/welcome");
    if !src.exists() {
        return Ok(false);
    }

    let dst = widgets_dir.join("welcome");
    copy_widget_dir(&src, &dst)?;
    Ok(true)
}

fn copy_widget_dir(src: &Path, dst: &Path) -> tauri::Result<()> {
    fs::create_dir_all(dst)?;
    for name in ["deskulpt.widget.json", "welcome.widget.js"] {
        let from = src.join(name);
        let to = dst.join(name);
        fs::copy(from, to)?;
    }
    Ok(())
}
