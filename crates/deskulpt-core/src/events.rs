//! Deskulpt core events.

use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use serde::Serialize;

use crate::config::WidgetCatalog;
use crate::settings::Settings;

/// Event for rendering a widget.
///
/// This event is emitted from the backend to the canvas window to instruct it
/// to render the specified widget.
#[derive(Debug, Serialize, specta::Type, Event)]
pub struct RenderWidgetEvent<'a> {
    /// The ID of the widget.
    pub id: &'a str,
    /// Either the code string to render or a bundling error message.
    pub code: Outcome<String>,
}

/// Event for showing a toast notification.
///
/// This event is emitted from the backend to the canvas window when a toast
/// notification needs to be displayed.
#[derive(Debug, Serialize, specta::Type, Event)]
#[serde(tag = "type", content = "content", rename_all = "camelCase")]
pub enum ShowToastEvent {
    /// Show a [success](https://sonner.emilkowal.ski/toast#success) toast.
    Success(String),
    /// Show an [error](https://sonner.emilkowal.ski/toast#error) toast.
    Error(String),
}

/// Event for updating the settings.
///
/// This event is emitted from the backend to all frontend windows whenever
/// there is a change in the settings. Full settings are included to ensure
/// that all windows see the most up-to-date version eventually.
#[derive(Debug, Serialize, specta::Type, Event)]
pub struct UpdateSettingsEvent<'a>(pub &'a Settings);

/// Event for updating the widget catalog.
///
/// This event is emitted from the backend to all frontend windows whenever
/// there is a change in the widget catalog.
#[derive(Debug, Serialize, specta::Type, Event)]
pub struct UpdateWidgetCatalogEvent<'a>(pub &'a WidgetCatalog);
