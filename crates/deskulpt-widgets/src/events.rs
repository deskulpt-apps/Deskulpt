use std::collections::HashMap;
use std::sync::Arc;

use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use serde::{Deserialize, Serialize};

use crate::catalog::WidgetCatalog;

/// Event for notifying the canvas to render widgets.
///
/// This event carries a mapping from widget IDs to their corresponding code
/// strings or bundling errors.
#[derive(Serialize, Deserialize, specta::Type, Event)]
pub struct RenderEvent(pub HashMap<String, Outcome<String>>);

/// Event for notifying frontends of a widgets update.
#[derive(Serialize, Deserialize, specta::Type, Event)]
pub struct UpdateEvent(pub Arc<WidgetCatalog>);
