use deskulpt_common::SerResult;
use tauri::{AppHandle, Runtime, command};

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
pub async fn refresh_widget<R: Runtime>(app_handle: AppHandle<R>, id: String) -> SerResult<()> {
    app_handle.reload_widget(&id)?;
    app_handle.render_widget(id)?;
    Ok(())
}
