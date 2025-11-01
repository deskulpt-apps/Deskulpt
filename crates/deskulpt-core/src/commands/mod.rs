//! Deskulpt core commands to be invoked by the frontend.

#[doc(hidden)]
mod call_plugin;
#[doc(hidden)]
mod complete_setup;
#[doc(hidden)]
mod open_widget;
#[doc(hidden)]
mod refresh_widget;
#[doc(hidden)]
mod refresh_widgets_all;
#[doc(hidden)]
mod update_settings;

pub use call_plugin::*;
pub use complete_setup::*;
pub use open_widget::*;
pub use refresh_widget::*;
pub use refresh_widgets_all::*;
pub use update_settings::*;
