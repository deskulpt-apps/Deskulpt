//! Tauri events.

use deskulpt_common::event::Event;
use serde::Serialize;

use crate::model::Settings;

/// Event for notifying frontend windows of a settings update.
#[derive(Debug, Serialize, specta::Type, Event)]
pub struct UpdateEvent<'a>(pub &'a Settings);
