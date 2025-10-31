use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::states::WidgetsStateExt;

/// TODO(Charlie-XIAO)
#[command]
#[specta::specta]
pub async fn refresh_widget<R: Runtime>(app_handle: AppHandle<R>, id: String) -> CmdResult<()> {
    app_handle.refresh_widget(&id)?;
    app_handle.render_widget(id)?;
    Ok(())
}
