//! Deskulpt widgets manager and its APIs.

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use anyhow::{Context, Result, anyhow, bail};
use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use deskulpt_core::path::PathExt;
use deskulpt_settings::{SettingsExt, SettingsPatch};
use parking_lot::{Mutex, RwLock};
use rolldown_watcher::{
    EventHandler, FileChangeResult, RecommendedWatcher, RecursiveMode, Watcher,
};
use tauri::{AppHandle, Manager, Runtime, WebviewWindow};
use tracing::{debug, error, info};

use crate::WidgetsExt;
use crate::catalog::{WidgetCatalog, WidgetDiscoveryHandler, WidgetManifest};
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
    /// Watcher for the widgets root directory.
    root_watcher: Mutex<Option<RecommendedWatcher>>,
    /// Watchers for individual widget directories keyed by widget ID.
    widget_watchers: RwLock<HashMap<String, RecommendedWatcher>>,
}

/// Generic file watcher handler that executes a callback on file changes.
struct WatcherHandler<R: Runtime> {
    app_handle: AppHandle<R>,
    action: Arc<dyn Fn(AppHandle<R>) + Send + Sync>,
    context: String,
}

impl<R: Runtime> EventHandler for WatcherHandler<R> {
    fn handle_event(&mut self, event: FileChangeResult) {
        if let Err(errors) = event {
            for error in errors {
                error!(error = ?error, context = %self.context, "Watcher error");
            }
            return;
        }

        let app_handle = self.app_handle.clone();
        let action = self.action.clone();
        tauri::async_runtime::spawn(async move {
            action(app_handle);
        });
    }
}

impl<R: Runtime> WidgetsManager<R> {
    /// Initialize the [`WidgetsManager`].
    ///
    /// The catalog is initialized as empty. When all windows complete setup,
    /// an initial refresh would be triggered by [`Self::complete_setup`] to
    /// populate the catalog. A render worker is started immediately.
    pub fn new(app_handle: AppHandle<R>) -> Self {
        let render_worker = RenderWorkerHandle::new(app_handle.clone());

        let manager = Self {
            app_handle,
            catalog: Default::default(),
            render_worker,
            setup_state: Default::default(),
            root_watcher: Default::default(),
            widget_watchers: Default::default(),
        };

        if let Err(error) = manager.start_root_watcher() {
            error!(?error, "Failed to start widgets root watcher");
        }

        manager
    }

    /// Start watching the widgets root directory for top-level changes.
    fn start_root_watcher(&self) -> Result<()> {
        let widgets_dir = self.app_handle.widgets_dir()?;
        let mut watcher = RecommendedWatcher::new(WatcherHandler {
            app_handle: self.app_handle.clone(),
            action: Arc::new(move |app| {
                if let Err(error) = app.widgets().reload_all() {
                    error!(?error, "Failed to reload all widgets after root change");
                }
            }),
            context: "root".to_string(),
        })?;
        watcher.watch(widgets_dir, RecursiveMode::NonRecursive)?;
        *self.root_watcher.lock() = Some(watcher);
        Ok(())
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
        if let Some(manifest) = manifest.transpose() {
            catalog.0.insert(id.to_string(), manifest.into());
            self.on_widget_discovered(id)?;
        } else {
            catalog.0.remove(id);
            self.widget_watchers.write().remove(id);
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
        let new_catalog = WidgetCatalog::load_with_handler(widgets_dir, self)?;
        let new_ids: HashSet<_> = new_catalog.0.keys().cloned().collect();

        self.widget_watchers
            .write()
            .retain(|id, _| new_ids.contains(id));

        {
            let mut catalog = self.catalog.write();
            *catalog = new_catalog;
            UpdateEvent(&catalog).emit(&self.app_handle)?;
        }

        self.app_handle.settings().update_with(|settings| {
            let catalog = self.catalog.read();
            catalog.compute_settings_patch(settings)
        })?;
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
    /// of all widgets is trigger via [`Self::refresh_all`].
    ///
    /// Tauri command: [`crate::commands::complete_setup`].
    pub fn complete_setup(&self, window: WebviewWindow<R>) -> Result<()> {
        let window = window.label().try_into().unwrap();
        let complete = self.setup_state.complete(window);
        if complete {
            self.refresh_all()?;
        }
        Ok(())
    }

    /// Add starter widgets if not already added.
    ///
    /// If the starter widgets have not been marked as added, this method will
    /// copy the starter widgets from the bundled resources to the widgets base
    /// directory. Failure to add individual starter widgets will be logged as
    /// errors, but will not prevent others from being added, and will not cause
    /// this method to return an error. However, only if all starter widgets are
    /// added successfully will the settings be updated to mark them as added.
    ///
    /// This method is idempotent. If all starter widgets have been successfully
    /// added once, subsequent calls are no-ops. If some starter widgets have
    /// been added but not all, subsequent calls will silently skip already
    /// existing starter widgets and attempt to add the remaining ones.
    pub fn maybe_add_starter(&self) -> Result<()> {
        if self.app_handle.settings().read().starter_widgets_added {
            return Ok(());
        }

        let resource_dir = self.app_handle.path().resource_dir()?;
        let widgets_dir = self.app_handle.widgets_dir()?;

        let mut has_error = false;
        for widget in ["welcome"] {
            let widget_id = format!("@deskulpt-starter.{widget}");
            let src = resource_dir.join(format!("resources/widgets/starter/{widget}"));
            let dst = widgets_dir.join(&widget_id);
            if dst.exists() {
                debug!(%widget_id, "Starter widget already exists, skipping");
                continue;
            }

            match copy_dir::copy_dir(&src, &dst)
                .with_context(|| format!("Failed to add starter widget {widget_id}"))
            {
                Ok(_) => {
                    info!(%widget_id, "Added starter widget");
                },
                Err(e) => {
                    has_error = true;
                    error!(
                        error = ?e,
                        %widget_id,
                        src = %src.display(),
                        dst = %dst.display(),
                        "Failed to add starter widget",
                    );
                },
            }
        }

        if !has_error {
            self.app_handle.settings().update(SettingsPatch {
                starter_widgets_added: Some(true),
                ..Default::default()
            })?;
        }
        Ok(())
    }
}

/// Creates a file watcher for each discovered widget.
impl<R: Runtime> WidgetDiscoveryHandler for WidgetsManager<R> {
    fn on_widget_discovered(&self, id: &str) -> Result<()> {
        match self.widget_watchers.write().entry(id.to_string()) {
            Entry::Occupied(_) => Ok(()),
            Entry::Vacant(e) => {
                let id_clone = id.to_string();
                let mut watcher = RecommendedWatcher::new(WatcherHandler {
                    app_handle: self.app_handle.clone(),
                    action: Arc::new(move |app| {
                        if let Err(error) = app.widgets().refresh(&id_clone) {
                            error!(?error, %id_clone, "Failed to refresh widget after change");
                        }
                    }),
                    context: format!("widget:{}", id),
                })?;
                watcher.watch(&self.app_handle.widget_dir(id)?, RecursiveMode::Recursive)?;
                e.insert(watcher);
                Ok(())
            },
        }
    }
}
