use deskulpt_common::SerResult;
use tauri::{AppHandle, Runtime, WebviewWindow, command};

use crate::commands::rescan_widgets;
use crate::states::SetupStateExt;

/// Mark the window to have completed its setup.
///
/// If all setup has been completed after marking this window as completed, this
/// command will automatically trigger an initial rescan of the widgets.
///
/// ### Errors
///
/// - Error rescanning the widgets (if applicable).
#[command]
#[specta::specta]
pub async fn complete_setup<R: Runtime>(
    app_handle: AppHandle<R>,
    window: WebviewWindow<R>,
) -> SerResult<()> {
    let window = window.label().try_into().unwrap();
    let complete = app_handle.complete_setup(window);
    if complete {
        rescan_widgets(app_handle).await?;
    }
    Ok(())
}
