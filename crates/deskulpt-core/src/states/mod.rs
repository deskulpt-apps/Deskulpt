//! Deskulpt runtime state management.

mod canvas_imode;
mod settings;
mod setup;
mod widget_catalog;

#[doc(hidden)]
pub use canvas_imode::CanvasImodeStateExt;
#[doc(hidden)]
pub use settings::SettingsStateExt;
#[doc(hidden)]
pub use setup::SetupStateExt;
#[doc(hidden)]
pub use widget_catalog::WidgetCatalogStateExt;
