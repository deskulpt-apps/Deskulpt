//! Deskulpt core commands to be invoked by the frontend.

#[doc(hidden)]
mod call_plugin;
#[doc(hidden)]
mod logs;
#[doc(hidden)]
mod open_widget;

pub use call_plugin::*;
pub use open_widget::*;
