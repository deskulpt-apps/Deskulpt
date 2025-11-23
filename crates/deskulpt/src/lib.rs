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
use serde_json::Value;
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
    let mut dir_names = vec![];
    for entry in fs::read_dir(&widgets_dir)?.filter_map(Result::ok) {
        if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
            let name = entry.file_name();
            let name = name.to_string_lossy().to_string();
            if !name.starts_with('.') {
                dir_names.push(name);
            }
        }
    }

    let mut needs_seed = dir_names.is_empty();
    if !dir_names.is_empty() {
        if dir_names.len() == 1 && dir_names[0] == "welcome" {
            let manifest_path = widgets_dir.join("welcome/deskulpt.widget.json");
            let manifest: Value =
                serde_json::from_reader(fs::File::open(&manifest_path)?).unwrap_or_default();
            let entry = manifest
                .get("entry")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let index_exists = widgets_dir.join("welcome/index.tsx").exists();
            needs_seed = entry != "index.tsx" || !index_exists;
            if needs_seed {
                println!(
                    "Updating bundled welcome widget to latest format in {}",
                    widgets_dir.display()
                );
            }
        } else {
            println!(
                "Skipping welcome widget seeding; widgets directory not empty (dirs found): {:?}",
                dir_names
            );
            return Ok(false);
        }
    }

    let resource_dir = app.path().resource_dir()?;
    let src = resource_dir.join("default-widgets/welcome");
    if !src.exists() {
        println!(
            "Skipping welcome widget seeding; bundled welcome not found at: {}",
            src.display()
        );
        return Ok(false);
    }

    let dst = widgets_dir.join("welcome");
    if needs_seed {
        println!(
            "Seeding bundled welcome widget from {} to {}",
            src.display(),
            dst.display()
        );
        copy_widget_dir(&src, &dst)?;
        return Ok(true);
    }

    Ok(false)
}

fn copy_widget_dir(src: &Path, dst: &Path) -> tauri::Result<()> {
    fs::create_dir_all(dst)?;
    for name in ["deskulpt.widget.json", "index.tsx"] {
        let from = src.join(name);
        let to = dst.join(name);
        fs::copy(from, to)?;
    }
    let legacy = dst.join("welcome.widget.js");
    if legacy.exists() {
        let _ = fs::remove_file(legacy);
    }
    Ok(())
}
