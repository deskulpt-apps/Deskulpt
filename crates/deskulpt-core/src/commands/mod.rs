//! Deskulpt core commands to be invoked by the frontend.

#[doc(hidden)]
mod call_plugin;
#[doc(hidden)]
mod logging;
#[doc(hidden)]
mod open_widget;

pub use call_plugin::*;
pub use logging::*;
pub use open_widget::*;
