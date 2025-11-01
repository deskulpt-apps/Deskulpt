//! State management for Deskulpt widgets.

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use anyhow::{anyhow, bail, Context, Result};
use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use deskulpt_common::window::DeskulptWindow;
use tauri::{App, AppHandle, Manager, Runtime};
use tokio::sync::mpsc::{self, UnboundedReceiver};

use crate::bundler::WidgetBundlerBuilder;
use crate::config::WidgetCatalog;
use crate::events::{RenderWidgetEvent, UpdateWidgetCatalogEvent};
use crate::path::PathExt;
use crate::states::SettingsStateExt;

/// Task to render a specific widget.
struct RenderWidgetTask {
    /// The widget ID.
    id: String,
    /// The entry file relative to the widget directory.
    entry: String,
}

/// Managed state for Deskulpt widgets.
struct WidgetsState {
    /// The widget catalog.
    catalog: RwLock<WidgetCatalog>,
    /// Sender for widget rendering tasks.
    render_tx: mpsc::UnboundedSender<RenderWidgetTask>,
}

/// Worker function to process widget rendering tasks.
async fn render_worker<R: Runtime>(
    app_handle: AppHandle<R>,
    mut rx: UnboundedReceiver<RenderWidgetTask>,
) {
    while let Some(task) = rx.recv().await {
        let code = async {
            let widget_dir = app_handle.widgets_dir()?.join(&task.id);
            let mut bundler = WidgetBundlerBuilder::new(widget_dir, task.entry).build()?;
            let code = bundler.bundle().await?;
            Ok::<_, anyhow::Error>(code)
        }
        .await
        .into();
        let event = RenderWidgetEvent { id: &task.id, code };
        if let Err(e) = event.emit_to(&app_handle, DeskulptWindow::Canvas) {
            eprintln!(
                "Failed to emit RenderWidgetEvent for widget {}: {e:?}",
                task.id
            );
        };
    }
}

/// Extension trait for operations on Deskulpt widgets.
pub trait WidgetsStateExt<R: Runtime>: Manager<R> + PathExt<R> + SettingsStateExt<R> {
    /// Initialize state management for Deskulpt widgets.
    fn manage_widgets(&self) {
        let (tx, rx) = mpsc::unbounded_channel::<RenderWidgetTask>();
        let app_handle = self.app_handle().clone();
        tauri::async_runtime::spawn(async move {
            render_worker(app_handle, rx).await;
        });

        self.manage(WidgetsState {
            catalog: Default::default(),
            render_tx: tx,
        });
    }

    /// Get an immutable reference to the widget catalog.
    ///
    /// The returned reference is behind a lock guard, which should be dropped
    /// as soon as possible to minimize critical section.
    fn get_widget_catalog(&self) -> RwLockReadGuard<'_, WidgetCatalog> {
        let state = self.state::<WidgetsState>().inner();
        state.catalog.read().unwrap()
    }

    /// Get a mutable reference to the widget catalog.
    ///
    /// The returned reference is behind a lock guard, which should be dropped
    /// as soon as possible to minimize critical section.
    fn get_widget_catalog_mut(&self) -> RwLockWriteGuard<'_, WidgetCatalog> {
        let state = self.state::<WidgetsState>().inner();
        state.catalog.write().unwrap()
    }

    /// Reload all widgets.
    ///
    /// This function loads a new widget catalog from the widgets directory and
    /// replaces the existing catalog. It then syncs the settings with the
    /// updated catalog. If any step fails, an error is returned.
    fn reload_widgets_all(&self) -> Result<()>
    where
        Self: Sized,
    {
        let widgets_dir = self.widgets_dir()?;
        let new_catalog = WidgetCatalog::load(widgets_dir)?;

        let state = self.state::<WidgetsState>();
        let mut catalog = state.catalog.write().unwrap();
        *catalog = new_catalog;
        UpdateWidgetCatalogEvent(&catalog).emit(self)?;

        self.update_settings(|settings| catalog.compute_settings_patch(settings))?;
        Ok(())
    }

    /// Render a specific widget by its ID.
    ///
    /// If the widget does not exist in the catalog, an error is returned.
    /// Otherwise, a render task is sent to the render worker. If this fails,
    /// an error is returned as well. This function does not wait for the
    /// rendering to complete.
    fn render_widget(&self, id: String) -> Result<()> {
        let state = self.state::<WidgetsState>();
        let catalog = state.catalog.read().unwrap();
        let config = catalog
            .0
            .get(&id)
            .ok_or_else(|| anyhow!("Widget {id} does not exist in the catalog"))?;

        if let Outcome::Ok(config) = config {
            state.render_tx.send(RenderWidgetTask {
                id,
                entry: config.entry.clone(),
            })?;
        }
        Ok(())
    }

    /// Render all widgets in the catalog.
    ///
    /// This function sends render tasks for all widgets in the catalog to the
    /// render worker. If any task fails to be sent, an error is returned
    /// containing all individual errors. This function does not wait for the
    /// rendering to complete.
    fn render_widgets_all(&self) -> Result<()> {
        let state = self.state::<WidgetsState>();
        let catalog = state.catalog.read().unwrap();

        let mut errors = vec![];
        for (id, config) in catalog.0.iter() {
            if let Outcome::Ok(config) = config {
                if let Err(e) = state
                    .render_tx
                    .send(RenderWidgetTask {
                        id: id.clone(),
                        entry: config.entry.clone(),
                    })
                    .with_context(|| format!("Failed to send render task for widget {id}"))
                {
                    errors.push(e);
                }
            }
        }

        if !errors.is_empty() {
            let message = errors
                .into_iter()
                .map(|e| format!("{e:?}"))
                .collect::<Vec<_>>()
                .join("\n");
            bail!(message);
        }

        Ok(())
    }
}

impl<R: Runtime> WidgetsStateExt<R> for App<R> {}
impl<R: Runtime> WidgetsStateExt<R> for AppHandle<R> {}
