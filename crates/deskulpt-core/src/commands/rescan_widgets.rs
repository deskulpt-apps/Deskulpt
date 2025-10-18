use deskulpt_common::event::Event;
use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::commands::bundle_widgets;
use crate::config::WidgetCatalog;
use crate::events::{UpdateSettingsEvent, UpdateWidgetCatalogEvent};
use crate::path::PathExt;
use crate::states::{SettingsStateExt, WidgetCatalogStateExt};

/// Rescan the widgets directory to discover widgets.
///
/// This command scans the widgets directory for available widgets and updates
/// the widget catalog and settings accordingly. It then emits events to notify
/// the frontend of these changes. Finally, it triggers the bundling of all
/// widgets in the updated catalog with `bundle_widgets` to ensure they are
/// ready for use.
///
/// ### Errors
///
/// - Error accessing the widgets directory.
/// - Error loading the new widget catalog from the widgets directory.
/// - Error emitting the [`UpdateSettingsEvent`].
/// - Error emitting the [`UpdateWidgetCatalogEvent`].
/// - Error bundling all discovered widgets.
#[command]
#[specta::specta]
pub async fn rescan_widgets<R: Runtime>(app_handle: AppHandle<R>) -> CmdResult<()> {
    let catalog = WidgetCatalog::load(app_handle.widgets_dir()?)?;
    *app_handle.get_widget_catalog_mut() = catalog.clone();

    {
        let mut settings = app_handle.get_settings_mut();
        settings.widgets.retain(|id, _| catalog.0.contains_key(id));
        for id in catalog.0.keys() {
            settings
                .widgets
                .entry(id.clone())
                .or_insert_with(Default::default);
        }
        UpdateSettingsEvent(settings.clone()).emit(&app_handle)?;
    }

    UpdateWidgetCatalogEvent(catalog).emit(&app_handle)?;

    bundle_widgets(app_handle, None).await?;
    Ok(())
}
