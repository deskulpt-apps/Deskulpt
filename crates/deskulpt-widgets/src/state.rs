use std::sync::{Arc, Mutex};

use anyhow::Result;
use deskulpt_common::event::Event;
use deskulpt_core::path::PathExt;
use deskulpt_core::states::SettingsStateExt;
use tauri::{AppHandle, Runtime};

use crate::catalog::WidgetCatalog;
use crate::events::UpdateEvent;

/// Managed state for Deskulpt widgets.
pub struct Widgets<R: Runtime> {
    /// An app handle embedded for convenience.
    app_handle: AppHandle<R>,
    /// The catalog of widgets.
    catalog: Mutex<Arc<WidgetCatalog>>,
}

impl<R: Runtime> Widgets<R> {
    /// Create a new [`Widgets`] instance.
    pub fn new(app_handle: AppHandle<R>) -> Self {
        Self {
            app_handle,
            catalog: Default::default(),
        }
    }

    /// Rescan the widgets directory to update the widget catalog.
    pub fn rescan(&self) -> Result<()> {
        let widgets_dir = self.app_handle.widgets_dir()?;
        let catalog = Arc::new(WidgetCatalog::scan(widgets_dir)?);

        let settings = self.app_handle.get_settings();
        let settings_patch = catalog.compute_settings_patch(&settings);
        drop(settings); // Prevent deadlock
        self.app_handle.apply_settings_patch(settings_patch)?;

        *self.catalog.lock().unwrap() = catalog.clone();
        UpdateEvent(catalog).emit(&self.app_handle)?;
        Ok(())
    }
}
