#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

// mod bundler;
mod catalog;
mod commands;
mod events;
mod state;

use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

use crate::state::Widgets;

deskulpt_common::bindings::build_bindings!();

/// Initialize the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    deskulpt_common::init::init_builder!()
        .setup(|app_handle, _| {
            app_handle.manage(Widgets::new(app_handle.clone()));
            Ok(())
        })
        .build()
}

/// Extension to [`Manager`] for accessing Deskulpt widgets APIs.
pub trait WidgetsExt<R: Runtime> {
    /// Get the managed state for Deskulpt widgets.
    fn widgets(&self) -> &Widgets<R>;
}

impl<R: Runtime, M: Manager<R>> WidgetsExt<R> for M {
    fn widgets(&self) -> &Widgets<R> {
        self.state::<Widgets<R>>().inner()
    }
}
