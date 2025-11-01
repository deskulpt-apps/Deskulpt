#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

use tauri::Runtime;
use tauri::plugin::TauriPlugin;

mod bundler;
mod commands;
mod config;
pub mod events;
pub mod path;
mod settings;
pub mod states;
pub mod tray;
pub mod window;

deskulpt_common::bindings::build_bindings!();

/// Initialize the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    deskulpt_common::init::init_builder!().build()
}

#[doc(hidden)]
pub mod schema {
    pub use crate::settings::Settings;
}
