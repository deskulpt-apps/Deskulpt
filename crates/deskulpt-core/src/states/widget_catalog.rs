//! State management for the widget catalog.

use std::sync::{RwLock, RwLockReadGuard};

use anyhow::Result;
use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use tauri::{App, AppHandle, Emitter, Manager, Runtime};

use crate::config::{WidgetCatalog, WidgetConfig};
use crate::events::UpdateWidgetCatalogEvent;
use crate::path::PathExt;
use crate::states::SettingsStateExt;

/// Managed state for the widget catalog.
#[derive(Default)]
struct WidgetCatalogState(RwLock<WidgetCatalog>);

/// Extension trait for operations on widget catalog state.
pub trait WidgetCatalogStateExt<R: Runtime>:
    Manager<R> + Emitter<R> + PathExt<R> + SettingsStateExt<R>
{
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

    fn refresh_widget(&self, id: &str) -> Result<()>
    where
        Self: Sized,
    {
        let widget_dir = self.widgets_dir()?.join(id);
        let config = WidgetConfig::load(&widget_dir);

        let state = self.state::<WidgetCatalogState>();
        let mut catalog = state.0.write().unwrap();
        match config {
            Ok(Some(config)) => {
                catalog.0.insert(id.to_string(), Outcome::Ok(config));
            },
            Ok(None) => {
                catalog.0.remove(id);
            },
            Err(e) => {
                catalog.0.insert(id.to_string(), Err(e).into());
            },
        };
        UpdateWidgetCatalogEvent(&catalog).emit(self)?;

        self.update_settings(|settings| catalog.compute_settings_patch(settings))?;
        Ok(())
    }

    fn refresh_all_widgets(&self) -> Result<()>
    where
        Self: Sized,
    {
        let widgets_dir = self.widgets_dir()?;
        let new_catalog = WidgetCatalog::load(&widgets_dir)?;

        let state = self.state::<WidgetCatalogState>();
        let mut catalog = state.0.write().unwrap();
        *catalog = new_catalog;
        UpdateWidgetCatalogEvent(&catalog).emit(self)?;

        self.update_settings(|settings| catalog.compute_settings_patch(settings))?;
        Ok(())
    }

    // #[allow(async_fn_in_trait)]
    // async fn bundle_widget(&self, id: &str) -> Result<()> {
    //     let catalog = self.get_widget_catalog();
    //     if let Some(Outcome::Ok(config)) = catalog.0.get(id) {
    //         let widget_dir = self.widgets_dir()?.join(id);
    //         let mut bundler =
    //             WidgetBundlerBuilder::new(widget_dir,
    // config.entry.clone()).build()?;         let code =
    // bundler.bundle().await?;         RenderWidgetsEvent(&HashMap::from([(id,
    // report)]))             .emit_to(&app_handle, DeskulptWindow::Canvas)?;
    //     }
    //     Ok(())
    // }
}

impl<R: Runtime> WidgetCatalogStateExt<R> for App<R> {}
impl<R: Runtime> WidgetCatalogStateExt<R> for AppHandle<R> {}
