//! Tauri events.

use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use serde::Serialize;

use crate::catalog::Catalog;

/// Event for rendering a widget.
///
/// This event is emitted from the backend to the canvas window to instruct it
/// to render the specified widget.
#[derive(Debug, Serialize, specta::Type, Event)]
pub struct RenderEvent<'a> {
    /// The ID of the widget.
    pub id: &'a str,
    /// Either the code string to render or a bundling error message.
    pub report: Outcome<String>,
}

/// Event for updating the widget catalog.
///
/// This event is emitted from the backend to all frontend windows whenever
/// there is a change in the widget catalog.
#[derive(Debug, Serialize, specta::Type, Event)]
pub struct UpdateEvent<'a>(pub &'a Catalog);
