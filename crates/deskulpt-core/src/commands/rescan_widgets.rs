use deskulpt_common::event::Event;
use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::commands::bundle_widgets;
use crate::config::WidgetCatalog;
use crate::events::UpdateWidgetCatalogEvent;
use crate::path::PathExt;
use crate::states::{SettingsStateExt, WidgetCatalogStateExt};

/// Rescan the widgets directory to discover widgets.
///
/// This command scans the widgets directory for available widgets and updates
/// the widget catalog. It also updates the settings in accordance with the new
/// catalog. It then emits an [`UpdateWidgetCatalogEvent`] to notify the
/// frontend of the catalog change. Finally, it triggers the bundling of all
/// widgets in the updated catalog with `bundle_widgets` to ensure they are
/// ready for use.
///
/// ### Errors
///
/// - Error accessing the widgets directory.
/// - Error loading the new widget catalog from the widgets directory.
/// - Error updating the settings based on the new catalog.
/// - Error emitting the [`UpdateWidgetCatalogEvent`].
/// - Error bundling all discovered widgets.
#[command]
#[specta::specta]
pub async fn rescan_widgets<R: Runtime>(app_handle: AppHandle<R>) -> CmdResult<()> {
    let catalog = WidgetCatalog::load(app_handle.widgets_dir()?)?;

    let patch = catalog.compute_settings_patch(&app_handle.get_settings());
    app_handle.apply_settings_patch(patch)?;

    {
        let mut prev_catalog = app_handle.get_widget_catalog_mut();
        *prev_catalog = catalog;
        UpdateWidgetCatalogEvent(&prev_catalog).emit(&app_handle)?;
    }

    bundle_widgets(app_handle, None).await?;
    Ok(())
}
