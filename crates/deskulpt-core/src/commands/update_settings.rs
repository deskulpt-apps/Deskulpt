use deskulpt_common::SerResult;
use tauri::{AppHandle, Runtime, command};

use crate::settings::SettingsPatch;
use crate::states::SettingsStateExt;

/// Wrapper of [`SettingsStateExt::update_settings`].
#[command]
#[specta::specta]
pub async fn update_settings<R: Runtime>(
    app_handle: AppHandle<R>,
    patch: SettingsPatch,
) -> SerResult<()> {
    app_handle.update_settings(|_| patch)?;
    Ok(())
}
