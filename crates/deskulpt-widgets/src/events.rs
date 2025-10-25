use deskulpt_common::event::Event;
use serde::{Deserialize, Serialize};

use crate::config::Catalog;

/// Event for updating the widget catalog.
///
/// This event is emitted from the backend to all frontend windows whenever
/// there is a change in the widget catalog.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct UpdateWidgetCatalogEvent(pub Catalog);
