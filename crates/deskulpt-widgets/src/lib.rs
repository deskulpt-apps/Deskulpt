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
            app_handle.init_widgets_dir()?;
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
    if app.settings().read().has_seen_starter_tutorial {
        return Ok(());
    }

    let src = app.path().resource_dir()?.join("default-widgets/welcome");
    let dst = app.widgets_dir()?.join("welcome");

    copy_dir::copy_dir(&src, &dst).map_err(|e| tauri::Error::Io(e.into()))?;

    Ok(())
}
