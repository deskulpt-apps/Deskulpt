use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::settings::SettingsPatch;
use crate::states::SettingsStateExt;

/// Wrapper of [`SettingsStateExt::apply_settings_patch`].
#[command]
#[specta::specta]
pub async fn update_settings<R: Runtime>(
    app_handle: AppHandle<R>,
    patch: SettingsPatch,
) -> CmdResult<()> {
    app_handle.apply_settings_patch(patch)?;
    Ok(())
}
