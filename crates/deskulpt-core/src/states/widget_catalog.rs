//! State management for the widget catalog.

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use tauri::{App, AppHandle, Manager, Runtime};

use crate::config::WidgetCatalog;
use crate::path::PathExt;

/// Managed state for the widget catalog.
#[derive(Default)]
struct WidgetCatalogState(RwLock<WidgetCatalog>);

/// Extension trait for operations on widget catalog state.
pub trait WidgetCatalogStateExt<R: Runtime>: Manager<R> + PathExt<R> {
    /// Initialize state management for the widget catalog.
    fn manage_widget_catalog(&self) {
        self.manage(WidgetCatalogState::default());
    }

    /// Get an immutable reference to the widget catalog.
    ///
    /// The returned reference is behind a lock guard, which should be dropped
    /// as soon as possible to minimize critical section.
    fn get_widget_catalog(&self) -> RwLockReadGuard<'_, WidgetCatalog> {
        let state = self.state::<WidgetCatalogState>().inner();
        state.0.read().unwrap()
    }

    /// Get a mutable reference to the widget catalog.
    ///
    /// The returned reference is behind a lock guard, which should be dropped
    /// as soon as possible to minimize critical section.
    fn get_widget_catalog_mut(&self) -> RwLockWriteGuard<'_, WidgetCatalog> {
        let state = self.state::<WidgetCatalogState>().inner();
        state.0.write().unwrap()
    }
}

impl<R: Runtime> WidgetCatalogStateExt<R> for App<R> {}
impl<R: Runtime> WidgetCatalogStateExt<R> for AppHandle<R> {}
