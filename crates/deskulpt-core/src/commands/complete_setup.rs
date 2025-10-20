use tauri::{command, AppHandle, Runtime, WebviewWindow};

use super::error::CmdResult;
use crate::commands::rescan_widgets;
use crate::states::SetupStateExt;

/// TODO
#[command]
#[specta::specta]
pub async fn complete_setup<R: Runtime>(
    app_handle: AppHandle<R>,
    window: WebviewWindow<R>,
) -> CmdResult<()> {
    let window = window.label().try_into().unwrap();
    let complete = app_handle.complete_setup(window);
    if complete {
        rescan_widgets(app_handle).await?;
    }
    Ok(())
}
