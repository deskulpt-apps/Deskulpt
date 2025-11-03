//! Deskulpt core commands to be invoked by the frontend.

#[doc(hidden)]
mod call_plugin;
#[doc(hidden)]
mod open_widget;
#[doc(hidden)]
mod update_settings;

pub use call_plugin::*;
pub use open_widget::*;
pub use update_settings::*;
