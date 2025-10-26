use std::sync::{Arc, Mutex};

use anyhow::Result;
use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use deskulpt_core::path::PathExt;
use deskulpt_core::states::SettingsStateExt;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime};

use crate::catalog::{WidgetCatalog, WidgetSpec};
use crate::events::UpdateEvent;

/// Specifies which widget(s) to bundle.
#[derive(Debug, Deserialize, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum BundleTarget {
    /// Bundle all widgets in the catalog.
    All,
    /// Bundle the specified widget.
    Id(String),
    /// Bundle the specified widgets.
    Ids(Vec<String>),
}

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
    ///
    /// TODO
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

    /// Bundle TODO
    pub fn bundle(&self, target: BundleTarget) -> Result<()> {
        let catalog = self.catalog.lock().unwrap().clone();
        let configs: Box<dyn Iterator<Item = (&String, &Outcome<WidgetSpec>)>> = match target {
            BundleTarget::All => Box::new(catalog.0.iter()),
            BundleTarget::Id(id) => Box::new(catalog.0.get_key_value(&id).into_iter()),
            BundleTarget::Ids(ids) => Box::new(
                ids.into_iter()
                    .map(|id| catalog.0.get_key_value(&id).expect("unknown id")),
            ),
        };

        todo!()
    }
}
