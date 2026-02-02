use anyhow::{Context, Result};
use deskulpt_settings::SettingsExt;
use deskulpt_settings::model::SettingsPatch;
use tauri::{Manager, Runtime};
use tracing::{debug, error, info};

use crate::WidgetsManager;

impl<R: Runtime> WidgetsManager<R> {
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
}
