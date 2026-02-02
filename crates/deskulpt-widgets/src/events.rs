//! Tauri events.

use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use serde::Serialize;

use crate::model::Widgets;

/// Event for reporting the rendering result of a widget to the canvas.
#[derive(Debug, Serialize, specta::Type, Event)]
pub struct RenderEvent<'a> {
    /// The ID of the widget.
    pub id: &'a str,
    /// Either the code string to render or a bundling error message.
    pub report: Outcome<String>,
}

/// Event for notifying frontend windows of a widget catalog update.
#[derive(Debug, Serialize, specta::Type, Event)]
pub struct UpdateEvent<'a>(pub &'a Widgets);
