use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::commands::bundle_widgets;
use crate::states::WidgetsStateExt;

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
/// - Error reloading all widgets.
/// - Error rendering all widgets.
#[command]
#[specta::specta]
pub async fn rescan_widgets<R: Runtime>(app_handle: AppHandle<R>) -> CmdResult<()> {
    app_handle.reload_widgets_all()?;
    bundle_widgets(app_handle, None).await?;
    Ok(())
}
