use deskulpt_common::SerResult;
use tauri::{AppHandle, Runtime, WebviewWindow};

use crate::WidgetsExt;
use crate::setup::SetupStateExt;

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
#[tauri::command]
#[specta::specta]
pub async fn bundle<R: Runtime>(app_handle: AppHandle<R>, id: Option<String>) -> SerResult<()> {
    match id {
        Some(id) => app_handle.widgets().render(id)?,
        None => app_handle.widgets().render_all()?,
    }
    Ok(())
}

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
#[tauri::command]
#[specta::specta]
pub async fn rescan<R: Runtime>(app_handle: AppHandle<R>) -> SerResult<()> {
    app_handle.widgets().reload_all()?;
    bundle(app_handle, None).await?;
    Ok(())
}

/// Mark the window to have completed its setup.
///
/// If all setup has been completed after marking this window as completed, this
/// command will automatically trigger an initial rescan of the widgets.
///
/// ### Errors
///
/// - Error rescanning the widgets (if applicable).
#[tauri::command]
#[specta::specta]
pub async fn complete_setup<R: Runtime>(
    app_handle: AppHandle<R>,
    window: WebviewWindow<R>,
) -> SerResult<()> {
    let window = window.label().try_into().unwrap();
    let complete = app_handle.complete_setup(window);
    if complete {
        rescan(app_handle).await?;
    }
    Ok(())
}
