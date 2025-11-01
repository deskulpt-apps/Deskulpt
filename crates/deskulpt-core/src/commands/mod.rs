//! Deskulpt core commands to be invoked by the frontend.

#[doc(hidden)]
mod bundle_widgets;
#[doc(hidden)]
mod call_plugin;
#[doc(hidden)]
mod complete_setup;
#[doc(hidden)]
mod open_widget;
#[doc(hidden)]
mod rescan_widgets;
#[doc(hidden)]
mod update_settings;

pub use bundle_widgets::*;
pub use call_plugin::*;
pub use complete_setup::*;
pub use open_widget::*;
pub use rescan_widgets::*;
pub use update_settings::*;
