#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

mod commands;
mod manager;
mod reader;

pub use manager::LogsManager;
pub use reader::{Cursor, Entry, Page};
use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

deskulpt_common::bindings::build_bindings!();

/// Initialize the internal Deskulpt logs plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    deskulpt_common::init::init_builder!()
        .setup(|app_handle, _| {
            app_handle.manage(LogsManager::new(app_handle.clone())?);
            Ok(())
        })
        .build()
}

/// Extension to [`Manager`] for accessing Deskulpt logs APIs.
pub trait LogsExt<R: Runtime> {
    /// Get a reference to the [`LogsManager`] to access the APIs.
    fn logs(&self) -> &LogsManager<R>;
}

impl<R: Runtime, M: Manager<R>> LogsExt<R> for M {
    fn logs(&self) -> &LogsManager<R> {
        self.state::<LogsManager<R>>().inner()
    }
}
