use deskulpt_common::SerResult;
use tauri::{AppHandle, Runtime, command};

use crate::states::WidgetsStateExt;

/// Bundle widget(s).
///
/// This command bundles the specified widget(s) that exist in the catalog. If
/// `id` is not provided, all widgets in the catalog are bundled. This only
/// notifies the bundler to process the widgets and does not wait for the
/// bundling to complete. Bundling results are communicated back to the canvas
/// window asynchronously.
///
/// ### Errors
///
/// - Error sending any bundling task to the bundler.
#[command]
#[specta::specta]
pub async fn bundle_widgets<R: Runtime>(
    app_handle: AppHandle<R>,
    id: Option<String>,
) -> SerResult<()> {
    match id {
        Some(id) => app_handle.render_widget(id)?,
        None => app_handle.render_widgets_all()?,
    }
    Ok(())
}
