#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

mod commands;
mod index;
mod manager;
mod widget;

use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

pub use crate::manager::RegistryManager;

deskulpt_common::bindings::build_bindings!();

/// Initialize the internal Deskulpt registry plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    deskulpt_common::init::init_builder!()
        .setup(|app_handle, _| {
            app_handle.manage(RegistryManager::new(app_handle.clone())?);
            Ok(())
        })
        .build()
}

/// Extension to [`Manager`] for accessing Deskulpt registry APIs.
pub trait RegistryExt<R: Runtime> {
    /// Get a reference to the [`WidgetsManager`] to access the APIs.
    fn registry(&self) -> &RegistryManager<R>;
}

impl<R: Runtime, M: Manager<R>> RegistryExt<R> for M {
    fn registry(&self) -> &RegistryManager<R> {
        self.state::<RegistryManager<R>>().inner()
    }
}
