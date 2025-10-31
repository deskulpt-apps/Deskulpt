use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::states::WidgetsStateExt;

/// TODO(Charlie-XIAO)
#[command]
#[specta::specta]
pub async fn refresh_all_widgets<R: Runtime>(app_handle: AppHandle<R>) -> CmdResult<()> {
    app_handle.refresh_widgets_all()?;
    app_handle.render_widgets_all()?;
    Ok(())
}
