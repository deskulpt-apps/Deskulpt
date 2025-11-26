//! Deskulpt widgets manager and its APIs.

use std::sync::Once;

use anyhow::{Result, anyhow, bail};
use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use deskulpt_core::path::PathExt;
use deskulpt_settings::{SettingsExt, SettingsPatch};
use parking_lot::RwLock;
use tauri::{AppHandle, Runtime, WebviewWindow};

use crate::catalog::{WidgetCatalog, WidgetManifest};
use crate::events::UpdateEvent;
use crate::render::{RenderWorkerHandle, RenderWorkerTask};
use crate::setup::SetupState;

/// Manager for Deskulpt widgets.
pub struct WidgetsManager<R: Runtime> {
    /// The Tauri app handle.
    app_handle: AppHandle<R>,
    /// The widget catalog.
    catalog: RwLock<WidgetCatalog>,
    /// The handle for the render worker.
    render_worker: RenderWorkerHandle,
    /// The setup state for frontend windows.
    setup_state: SetupState,
    /// Ensures the initial refresh only runs once.
    refresh_once: Once,
}

impl<R: Runtime> WidgetsManager<R> {
    /// Initialize the [`WidgetsManager`].
    ///
    /// The catalog is initialized as empty. When all windows complete setup,
    /// an initial refresh would be triggered by [`Self::complete_setup`] to
    /// populate the catalog. A render worker is started immediately.
    pub fn new(app_handle: AppHandle<R>) -> Self {
        let render_worker = RenderWorkerHandle::new(app_handle.clone());

        Self {
            app_handle,
            catalog: Default::default(),
            render_worker,
            setup_state: Default::default(),
            refresh_once: Once::new(),
        }
    }

    /// Reload a specific widget by its ID.
    ///
    /// This method loads the widget manifest from the corresponding widget
    /// directory and updates the catalog entry for that widget. This could be
    /// an addition, removal, or modification. It then syncs the settings with
    /// the updated catalog. If any step fails, an error is returned.
    pub fn reload(&self, id: &str) -> Result<()> {
        let widget_dir = self.app_handle.widgets_dir()?.join(id);
        let manifest = WidgetManifest::load(&widget_dir);

        let mut catalog = self.catalog.write();
        let has_seen = self.app_handle.settings().read().has_seen_starter_tutorial;
        if id == "welcome" {
            if has_seen {
                catalog.0.remove(id);
                UpdateEvent(&catalog).emit(&self.app_handle)?;
                self.app_handle
                    .settings()
                    .update_with(|settings| catalog.compute_settings_patch(settings))?;
                return Ok(());
            } else {
                self.app_handle.settings().update_with(|_| SettingsPatch {
                    has_seen_starter_tutorial: Some(true),
                    ..Default::default()
                })?;
            }
        }
        if let Some(manifest) = manifest.transpose() {
            catalog.0.insert(id.to_string(), manifest.into());
        } else {
            catalog.0.remove(id);
        }
        UpdateEvent(&catalog).emit(&self.app_handle)?;

        self.app_handle
            .settings()
            .update_with(|settings| catalog.compute_settings_patch(settings))?;
        Ok(())
    }

    /// Reload all widgets.
    ///
    /// This method loads a new widget catalog from the widgets directory and
    /// replaces the existing catalog. It then syncs the settings with the
    /// updated catalog. If any step fails, an error is returned.
    pub fn reload_all(&self) -> Result<()> {
        let widgets_dir = self.app_handle.widgets_dir()?;
        let mut new_catalog = WidgetCatalog::load(widgets_dir)?;
        let has_seen = self.app_handle.settings().read().has_seen_starter_tutorial;
        if has_seen {
            new_catalog.0.remove("welcome");
        } else if new_catalog.0.contains_key("welcome") {
            self.app_handle.settings().update_with(|_| SettingsPatch {
                has_seen_starter_tutorial: Some(true),
                ..Default::default()
            })?;
        }

        let mut catalog = self.catalog.write();
        *catalog = new_catalog;
        UpdateEvent(&catalog).emit(&self.app_handle)?;

        self.app_handle
            .settings()
            .update_with(|settings| catalog.compute_settings_patch(settings))?;
        Ok(())
    }

    /// Render a specific widget by its ID.
    ///
    /// This method submits a render task for the specified widget to the render
    /// worker. If the widget does not exist in the catalog or if task
    /// submission fails, an error is returned. This method is non-blocking and
    /// does not wait for the task to complete.
    pub fn render(&self, id: &str) -> Result<()> {
        let catalog = self.catalog.read();
        let config = catalog
            .0
            .get(id)
            .ok_or_else(|| anyhow!("Widget {id} does not exist in the catalog"))?;

        if let Outcome::Ok(config) = config {
            self.render_worker.process(RenderWorkerTask::Render {
                id: id.to_string(),
                entry: config.entry.clone(),
            })?;
        }
        Ok(())
    }

    /// Render all widgets in the catalog.
    ///
    /// This method submits render tasks for all widgets in the catalog to the
    /// render worker. If any task submission fails, an error containing all
    /// accumulated errors is returned. This method is non-blocking and does not
    /// wait for the tasks to complete.
    pub fn render_all(&self) -> Result<()> {
        let catalog = self.catalog.read();

        let mut errors = vec![];
        for (id, config) in catalog.0.iter() {
            if let Outcome::Ok(config) = config
                && let Err(e) = self.render_worker.process(RenderWorkerTask::Render {
                    id: id.clone(),
                    entry: config.entry.clone(),
                })
            {
                errors.push(e.context(format!("Failed to send render task for widget {id}")));
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

    /// Refresh a specific widget by its ID.
    ///
    /// This is equivalent to reloading that widget with [`Self::reload`] then
    /// rendering it with [`Self::render`].
    ///
    /// Tauri command: [`crate::commands::refresh`].
    pub fn refresh(&self, id: &str) -> Result<()> {
        self.reload(id)?;
        self.render(id)?;
        Ok(())
    }

    /// Refresh all widgets.
    ///
    /// This is equivalent to reloading all widgets with [`Self::reload_all`]
    /// then rendering all widgets with [`Self::render_all`].
    ///
    /// Tauri command: [`crate::commands::refresh_all`].
    pub fn refresh_all(&self) -> Result<()> {
        self.reload_all()?;
        self.render_all()?;
        Ok(())
    }

    /// Mark a window as having completed setup.
    ///
    /// If all windows have completed setup after this call, an initial refresh
    /// of all widgets is trigger via [`Self::refresh_all`]. This refresh will
    /// only run once, even if this method is called multiple times.
    ///
    /// Tauri command: [`crate::commands::complete_setup`].
    pub fn complete_setup(&self, window: WebviewWindow<R>) -> Result<()> {
        let window = window.label().try_into().unwrap();
        let complete = self.setup_state.complete(window);
        if complete {
            self.refresh_once.call_once(|| {
                let _ = self.refresh_all();
            });
        }
        Ok(())
    }
}
