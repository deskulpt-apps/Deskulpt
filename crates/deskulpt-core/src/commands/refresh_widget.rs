use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::states::WidgetsStateExt;

/// Refresh a specific widget.
///
/// This command reloads the specified widget and triggers its rendering.
///
/// ### Errors
///
/// - Error reloading the widget.
/// - Error rendering the widget.
#[command]
#[specta::specta]
pub async fn refresh_widget<R: Runtime>(app_handle: AppHandle<R>, id: String) -> CmdResult<()> {
    app_handle.reload_widget(&id)?;
    app_handle.render_widget(id)?;
    Ok(())
}
