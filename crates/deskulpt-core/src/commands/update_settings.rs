use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::settings::SettingsPatch;
use crate::states::SettingsStateExt;

/// Wrapper of [`SettingsStateExt::update_settings`].
#[command]
#[specta::specta]
pub async fn update_settings<R: Runtime>(
    app_handle: AppHandle<R>,
    patch: SettingsPatch,
) -> CmdResult<()> {
    app_handle.update_settings(|_| patch)?;
    Ok(())
}
