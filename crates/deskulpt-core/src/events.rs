//! Deskulpt core events.

use std::collections::HashMap;

use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use serde::{Deserialize, Serialize};

use crate::config::WidgetCatalog;
use crate::settings::Settings;

/// Event for rendering widgets.
///
/// This event is emitted from the backend to the canvas window to instruct it
/// to render the provided widgets. The event carries a mapping from widget IDs
/// to their corresponding code strings.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct RenderWidgetsEvent(pub HashMap<String, Outcome<String>>);

/// Event for showing a toast notification.
///
/// This event is emitted from the backend to the canvas window when a toast
/// notification needs to be displayed.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
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
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct UpdateSettingsEvent(pub Settings);

/// Event for updating the widget catalog.
///
/// This event is emitted from the backend to all frontend windows whenever
/// there is a change in the widget catalog.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct UpdateWidgetCatalogEvent(pub WidgetCatalog);
