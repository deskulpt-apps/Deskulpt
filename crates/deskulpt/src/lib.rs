#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

use deskulpt_core::shortcuts::ShortcutsExt;
use deskulpt_core::states::CanvasImodeStateExt;
use deskulpt_core::tray::TrayExt;
use deskulpt_core::window::WindowExt;
use deskulpt_widgets::WidgetsExt;
use tauri::{Builder, generate_context};

/// Entry point for the Deskulpt backend.
pub fn run() {
    Builder::default()
        .setup(move |app| {
            // Hide the application from the dock on macOS because skipping
            // taskbar is not applicable for macOS
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            app.init_shortcuts();
            app.create_canvas()?;
            app.create_tray()?;

            app.manage_canvas_imode()?;

            app.widgets().ensure_starter()?;

            Ok(())
        })
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
        .plugin(deskulpt_logs::init())
        .run(generate_context!())
        .expect("Error running the Deskulpt application");
}
