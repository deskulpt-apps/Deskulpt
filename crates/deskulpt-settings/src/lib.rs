#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

mod commands;
mod events;
mod manager;
mod settings;
mod worker;

pub use manager::SettingsManager;
#[doc(no_inline)] // Avoid duplicate docs
pub use settings::{
    CanvasImode, Settings, SettingsPatch, ShortcutAction, Theme, WidgetSettings,
    WidgetSettingsPatch,
};
use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

deskulpt_common::bindings::build_bindings!();

/// Initialize the internal Deskulpt settings plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    deskulpt_common::init::init_builder!()
        .setup(|app_handle, _| {
            app_handle.manage(SettingsManager::new(app_handle.clone())?);
            Ok(())
        })
        .build()
}

/// Extension to [`Manager`] for accessing Deskulpt settings APIs.
pub trait SettingsExt<R: Runtime> {
    /// Get a reference to the [`SettingsManager`] to access the APIs.
    fn settings(&self) -> &SettingsManager<R>;
}

impl<R: Runtime, M: Manager<R>> SettingsExt<R> for M {
    fn settings(&self) -> &SettingsManager<R> {
        self.state::<SettingsManager<R>>().inner()
    }
}
