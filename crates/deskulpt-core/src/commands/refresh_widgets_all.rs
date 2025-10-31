use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::states::WidgetsStateExt;

/// Refresh all widgets.
///
/// This command reloads all widgets and triggers their rendering.
///
/// ### Errors
///
/// - Error reloading the widgets.
/// - Error rendering the widgets.
#[command]
#[specta::specta]
pub async fn refresh_widgets_all<R: Runtime>(app_handle: AppHandle<R>) -> CmdResult<()> {
    app_handle.reload_widgets_all()?;
    app_handle.render_widgets_all()?;
    Ok(())
}
