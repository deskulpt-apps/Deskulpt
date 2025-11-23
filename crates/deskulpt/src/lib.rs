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

fn seed_welcome_widget_if_empty<R: tauri::Runtime>(app: &tauri::App<R>) -> tauri::Result<()> {
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
            return Ok(());
        }
    }

    if !needs_seed {
        return Ok(());
    }

    let resource_dir = app.path().resource_dir()?;
    let src = resource_dir.join("default-widgets/welcome");

    if src.exists() {
        let dst = widgets_dir.join("welcome");
        if dst.exists() {
            fs::remove_dir_all(&dst)?;
        }
        println!(
            "Seeding bundled welcome widget from {} to {}",
            src.display(),
            dst.display()
        );
        copy_welcome_files(&src, &dst)?;
    } else {
        println!(
            "Skipping welcome widget seeding; bundled welcome not found at: {}",
            src.display()
        );
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn copy_welcome_files(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for name in [
        "deskulpt.widget.json",
        "index.tsx",
        "deskulpt.svg",
        "deskulpt-dark.svg",
    ] {
        let from = src.join(name);
        let to = dst.join(name);
        if from.exists() {
            fs::copy(from, to)?;
        }
    }
    let legacy_js = dst.join("index.js");
    if legacy_js.exists() {
        let _ = fs::remove_file(legacy_js);
    }
    Ok(())
}
