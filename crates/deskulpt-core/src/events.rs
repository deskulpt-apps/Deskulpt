//! Deskulpt core events.

use deskulpt_common::event::Event;
use serde::{Deserialize, Serialize};

use crate::settings::Settings;

/// Event for removing widgets.
///
/// This event is emitted from the manager window to the canvas window when
/// widgets need to be removed.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct RemoveWidgetsEvent(
    /// The list of widget IDs to be removed.
    Vec<String>,
);

/// Event for re-rendering widgets.
///
/// This event is mainly emitted from the manager window to the canvas window
/// when settings or code of a widget needs to be re-rendered. It may also be
/// emitted from the backend to the canvas window for the initial render.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct RenderWidgetsEvent(
    /// The list of widget IDs to be re-rendered.
    Vec<String>,
);

impl RenderWidgetsEvent {
    /// Create a new render widgets event for the given widget IDs.
    pub fn new(ids: Vec<String>) -> Self {
        Self(ids)
    }
}

impl From<Vec<String>> for RenderWidgetsEvent {
    fn from(ids: Vec<String>) -> Self {
        Self::new(ids)
    }
}

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

/// Event for updating settings of a widget.
///
/// This event is emitted between the manager window and the canvas window to
/// each other when widget settings are updated on one side.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct UpdateSettingsEvent(pub Settings);
