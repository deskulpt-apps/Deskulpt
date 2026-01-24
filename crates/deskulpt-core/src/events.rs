//! Deskulpt core events.

use deskulpt_common::event::Event;
use serde::Serialize;

/// Event for showing a toast notification.
///
/// This event is emitted from the backend to the canvas when a toast
/// notification needs to be displayed.
#[derive(Debug, Serialize, specta::Type, Event)]
#[serde(tag = "type", content = "content", rename_all = "camelCase")]
pub enum ShowToastEvent {
    /// Show a [success](https://sonner.emilkowal.ski/toast#success) toast.
    Success(String),
    /// Show an [error](https://sonner.emilkowal.ski/toast#error) toast.
    Error(String),
}
