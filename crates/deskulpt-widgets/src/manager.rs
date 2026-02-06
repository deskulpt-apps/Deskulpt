//! Deskulpt widgets manager and its APIs.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, bail};
use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use deskulpt_settings::SettingsExt;
use deskulpt_settings::model::SettingsPatch;
use parking_lot::RwLock;
use tauri::{AppHandle, Manager, Runtime};
use tracing::{debug, error, info};

use crate::catalog::{WidgetCatalog, WidgetSettingsPatch};
use crate::events::UpdateEvent;
use crate::persist::{PersistWorkerHandle, PersistedWidgetCatalog, PersistedWidgetCatalogView};
use crate::registry::{
    RegistryIndex, RegistryIndexFetcher, RegistryWidgetFetcher, RegistryWidgetPreview,
    RegistryWidgetReference,
};
use crate::render::{RenderWorkerHandle, RenderWorkerTask};

/// Manager for Deskulpt widgets.
pub struct WidgetsManager<R: Runtime> {
    /// The Tauri app handle.
    app_handle: AppHandle<R>,
    /// The widgets directory.
    dir: PathBuf,
    /// The widget catalog.
    catalog: RwLock<WidgetCatalog>,
    /// The path where widgets are persisted.
    persist_path: PathBuf,
    /// The handle for the render worker.
    render_worker: RenderWorkerHandle,
    /// The handle for the persist worker.
    persist_worker: PersistWorkerHandle,
}

impl<R: Runtime> WidgetsManager<R> {
    /// Initialize the [`WidgetsManager`].
    ///
    /// The catalog will be populated with widgets in the widgets directory and
    /// the persisted settings file. A render worker and a persist worker will
    /// be started immediately.
    pub fn new(app_handle: AppHandle<R>) -> Result<Self> {
        let dir = if cfg!(debug_assertions) {
            app_handle.path().resource_dir()?
        } else {
            app_handle.path().document_dir()?.join("Deskulpt")
        };
        let dir = dunce::simplified(&dir).join("widgets");
        std::fs::create_dir_all(&dir)?;

        let mut catalog = WidgetCatalog::default();
        catalog.reload_all(&dir)?;

        let persist_path = app_handle.path().app_local_data_dir()?.join("widgets.json");
        let mut persisted_catalog =
            PersistedWidgetCatalog::load(&persist_path).unwrap_or_else(|e| {
                error!("Failed to load persisted widgets: {e:?}");
                Default::default()
            });
        catalog.0.iter_mut().for_each(|(k, v)| {
            if let Some(persisted) = persisted_catalog.0.remove(k) {
                v.settings = persisted.settings;
            }
        });

        let render_worker = RenderWorkerHandle::new(app_handle.clone());
        let persist_worker = PersistWorkerHandle::new(app_handle.clone())?;

        Ok(Self {
            app_handle,
            dir,
            catalog: RwLock::new(catalog),
            persist_path,
            render_worker,
            persist_worker,
        })
    }

    /// Get the widgets directory.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Update the settings of a widget with a patch.
    ///
    /// An error is returned if the widget does not exist.
    pub fn update_settings(&self, id: &str, patch: WidgetSettingsPatch) -> Result<()> {
        let mut catalog = self.catalog.write();
        let widget = catalog
            .0
            .get_mut(id)
            .ok_or_else(|| anyhow!("Widget not found: {id}"))?;

        let changed = widget.settings.apply_patch(patch);
        if changed {
            UpdateEvent(&catalog).emit(&self.app_handle)?;
            self.persist_worker.notify()?;
        }
        Ok(())
    }

    /// Try to check if a point is covered by any widget geometrically.
    ///
    /// This method is non-blocking and might return `None` if the widget
    /// catalog is currently locked for writing.
    pub fn try_covers_point(&self, x: f64, y: f64) -> Option<bool> {
        let catalog = self.catalog.try_read()?;
        let covers = catalog
            .0
            .values()
            .any(|widget| widget.settings.covers_point(x, y));
        Some(covers)
    }

    /// Persist the current widgets to disk.
    pub fn persist(&self) -> Result<()> {
        let catalog = self.catalog.read();
        let view: PersistedWidgetCatalogView = (&*catalog).into();
        view.persist(&self.persist_path)?;
        Ok(())
    }

    /// Reload a specific widget by its ID.
    ///
    /// This method loads the widget manifest from the corresponding widget
    /// directory and updates the catalog entry for that widget. This could be
    /// an addition, removal, or modification. It then syncs the settings with
    /// the updated catalog. If any step fails, an error is returned.
    pub fn reload(&self, id: &str) -> Result<()> {
        let widget_dir = self.dir.join(id);

        let mut catalog = self.catalog.write();
        catalog.reload(&widget_dir, id)?;

        UpdateEvent(&catalog).emit(&self.app_handle)?;
        self.persist_worker.notify()?;
        Ok(())
    }

    /// Reload all widgets.
    ///
    /// This method loads a new widget catalog from the widgets directory and
    /// replaces the existing catalog. It then syncs the settings with the
    /// updated catalog. If any step fails, an error is returned.
    pub fn reload_all(&self) -> Result<()> {
        let mut catalog = self.catalog.write();
        catalog.reload_all(&self.dir)?;

        UpdateEvent(&catalog).emit(&self.app_handle)?;
        self.persist_worker.notify()?;
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
        let widget = catalog
            .0
            .get(id)
            .ok_or_else(|| anyhow!("Widget {id} does not exist in the catalog"))?;

        if let Outcome::Ok(manifest) = &widget.manifest {
            self.render_worker.process(RenderWorkerTask::Render {
                id: id.to_string(),
                entry: manifest.entry.clone(),
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
        for (id, widget) in catalog.0.iter() {
            if let Outcome::Ok(manifest) = &widget.manifest
                && let Err(e) = self.render_worker.process(RenderWorkerTask::Render {
                    id: id.clone(),
                    entry: manifest.entry.clone(),
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

        let mut has_error = false;
        for widget in ["welcome"] {
            let widget_id = format!("@deskulpt-starter.{widget}");
            let src = resource_dir
                .join("resources")
                .join("widgets")
                .join("starter")
                .join(widget);
            let dst = self.dir.join(&widget_id);
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

    /// Fetch the widgets registry index.
    ///
    /// Before fetching, this method ensures that the catalog is up-to-date by
    /// reloading all widgets. This is necessary for the frontend to know which
    /// widgets are already installed.
    pub async fn fetch_registry_index(&self) -> Result<RegistryIndex> {
        self.reload_all()?;

        let cache_dir = self.app_handle.path().app_cache_dir()?;
        let fetcher = RegistryIndexFetcher::new(&cache_dir);
        fetcher.fetch().await
    }

    /// Preview a widget from the registry.
    pub async fn preview(&self, widget: &RegistryWidgetReference) -> Result<RegistryWidgetPreview> {
        RegistryWidgetFetcher::default().preview(widget).await
    }

    /// Install a widget from the registry.
    ///
    /// If the widget already exists locally, an error is returned. After
    /// installation, the widget is automatically refreshed to update the
    /// catalog and render it.
    pub async fn install(&self, widget: &RegistryWidgetReference) -> Result<()> {
        let id = widget.local_id();
        let widget_dir = self.dir.join(&id);
        if widget_dir.exists() {
            bail!("Widget {id} already installed");
        }

        RegistryWidgetFetcher::default()
            .install(&widget_dir, widget)
            .await?;

        self.refresh(&id)?;
        Ok(())
    }

    /// Uninstall a widget from the registry.
    ///
    /// If the widget does not exist locally, an error is returned. After
    /// uninstallation, the widget is automatically reloaded to remove it from
    /// the catalog.
    pub async fn uninstall(&self, widget: &RegistryWidgetReference) -> Result<()> {
        let id = widget.local_id();
        let widget_dir = self.dir.join(&id);
        if !widget_dir.exists() {
            bail!("Widget {id} is not installed");
        }
        tokio::fs::remove_dir_all(&widget_dir)
            .await
            .with_context(|| format!("Failed to remove directory {}", widget_dir.display()))?;

        self.reload(&id)?;
        Ok(())
    }

    /// Upgrade a widget from the registry.
    ///
    /// If the widget does not exist locally, an error is returned. After
    /// upgrading, the widget is automatically refreshed to update the catalog
    /// and render it.
    pub async fn upgrade(&self, widget: &RegistryWidgetReference) -> Result<()> {
        let id = widget.local_id();
        let widget_dir = self.dir.join(&id);
        if !widget_dir.exists() {
            bail!("Widget {id} is not installed");
        }

        // TODO: We should ideally perform some form of backup to allow rollback
        // on failure, to avoid leaving the widget in a broken state
        tokio::fs::remove_dir_all(&widget_dir)
            .await
            .with_context(|| format!("Failed to remove directory {}", widget_dir.display()))?;

        RegistryWidgetFetcher::default()
            .install(&widget_dir, widget)
            .await?;

        self.refresh(&id)?;
        Ok(())
    }
}
