//! Tauri commands.

use deskulpt_common::SerResult;
use tauri::{AppHandle, Runtime, WebviewWindow};

use crate::WidgetsExt;
use crate::setup::SetupStateExt;

/// Wrapper of [`crate::Widgets::bundle`].
#[tauri::command]
#[specta::specta]
pub async fn bundle<R: Runtime>(app_handle: AppHandle<R>, id: Option<String>) -> SerResult<()> {
    app_handle.widgets().bundle(id)?;
    Ok(())
}

/// Wrapper of [`crate::Widgets::rescan`].
#[tauri::command]
#[specta::specta]
pub async fn rescan<R: Runtime>(app_handle: AppHandle<R>) -> SerResult<()> {
    app_handle.widgets().rescan()?;
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
