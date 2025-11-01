//! Deskulpt core events.

use deskulpt_common::event::Event;
use serde::Serialize;

use crate::settings::Settings;

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
