#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

use tauri::Runtime;
use tauri::plugin::TauriPlugin;

pub mod commands;
pub mod events;
pub mod path;
pub mod shortcuts;
pub mod states;
pub mod tray;
pub mod window;

deskulpt_common::bindings::build_bindings!();

/// Initialize the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    deskulpt_common::init::init_builder!().build()
}
