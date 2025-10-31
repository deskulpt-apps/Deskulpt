use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::states::WidgetCatalogStateExt;

/// TODO(Charlie-XIAO)
#[command]
#[specta::specta]
pub async fn refresh_all_widgets<R: Runtime>(app_handle: AppHandle<R>) -> CmdResult<()> {
    app_handle.refresh_all_widgets()?;
    app_handle.render_widgets(None).await?;
    Ok(())
}
