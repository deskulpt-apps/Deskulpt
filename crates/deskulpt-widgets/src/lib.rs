#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

mod catalog;
mod commands;
mod events;
mod manager;
mod render;
mod setup;

use std::fs;
use std::path::Path;

use deskulpt_core::path::PathExt;
use deskulpt_settings::SettingsExt;
pub use manager::WidgetsManager;
use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

deskulpt_common::bindings::build_bindings!();

/// Initialize the internal Deskulpt widgets plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    deskulpt_common::init::init_builder!()
        .setup(|app_handle, _| {
            seed_welcome_widget_if_needed(app_handle)?;
            app_handle.manage(WidgetsManager::new(app_handle.clone()));
            Ok(())
        })
        .build()
}

/// Extension to [`Manager`] for accessing Deskulpt widgets APIs.
trait WidgetsExt<R: Runtime> {
    /// Get a reference to the [`WidgetsManager`] to access the APIs.
    fn widgets(&self) -> &WidgetsManager<R>;
}

impl<R: Runtime, M: Manager<R>> WidgetsExt<R> for M {
    fn widgets(&self) -> &WidgetsManager<R> {
        self.state::<WidgetsManager<R>>().inner()
    }
}

fn seed_welcome_widget_if_needed<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    // Check the flag from settings
    let has_seen = app.settings().read().has_seen_starter_tutorial;
    if has_seen {
        return Ok(());
    }

    // Seed the welcome widget
    let widgets_dir = app.widgets_dir()?;
    let resource_dir = app.path().resource_dir()?;
    let src = resource_dir.join("default-widgets/welcome");

    if src.exists() {
        let dst = widgets_dir.join("welcome");
        if !dst.exists() {
            println!(
                "Seeding bundled welcome widget from {} to {}",
                src.display(),
                dst.display()
            );
            copy_welcome_files(&src, &dst)?;
        }
    }

    Ok(())
}

fn copy_welcome_files(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
