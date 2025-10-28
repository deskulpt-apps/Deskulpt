use std::sync::Arc;

use deskulpt_common::event::Event;
use serde::{Deserialize, Serialize};

use crate::catalog::WidgetCatalog;

/// Event for notifying frontends of a widgets update.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct UpdateEvent(pub Arc<WidgetCatalog>);
