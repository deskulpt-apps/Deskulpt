use anyhow::{Context, Result, bail};
use deskulpt_registry::{Index, IndexFetcher, WidgetFetcher, WidgetPreview, WidgetReference};
use tauri::{Manager, Runtime};

use crate::WidgetsManager;

impl<R: Runtime> WidgetsManager<R> {
    /// Fetch the widgets registry index.
    ///
    /// Before fetching, this method ensures that the catalog is up-to-date by
    /// reloading all widgets. This is necessary for the frontend to know which
    /// widgets are already installed.
    pub(crate) async fn fetch_registry_index(&self) -> Result<Index> {
        self.reload_all()?;

        let cache_dir = self.app_handle.path().app_cache_dir()?;
        let fetcher = IndexFetcher::new(&cache_dir);
        fetcher.fetch().await
    }

    /// Preview a widget from the registry.
    pub(crate) async fn preview(&self, widget: &WidgetReference) -> Result<WidgetPreview> {
        WidgetFetcher::default().preview(widget).await
    }

    /// Install a widget from the registry.
    ///
    /// If the widget already exists locally, an error is returned. After
    /// installation, the widget is automatically refreshed to update the
    /// catalog and render it.
    pub(crate) async fn install(&self, widget: &WidgetReference) -> Result<()> {
        let id = widget.local_id();
        let widget_dir = self.dir.join(&id);
        if widget_dir.exists() {
            bail!("Widget {id} already installed");
        }

        WidgetFetcher::default()
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
    pub(crate) async fn uninstall(&self, widget: &WidgetReference) -> Result<()> {
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
    pub(crate) async fn upgrade(&self, widget: &WidgetReference) -> Result<()> {
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

        WidgetFetcher::default()
            .install(&widget_dir, widget)
            .await?;

        self.refresh(&id)?;
        Ok(())
    }
}
