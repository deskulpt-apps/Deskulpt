use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::settings::SettingsPatch;
use crate::states::SettingsStateExt;

/// Update the settings.
///
/// This command updates the settings state in the backend. If an update has
/// side effects, they will be applied prior to the update being committed. See
/// [`SettingsStateExt`] for more information.
///
/// ### Errors
///
/// - Failed to apply the side effects, if any.
#[command]
#[specta::specta]
pub async fn update_settings<R: Runtime>(
    app_handle: AppHandle<R>,
    patch: SettingsPatch,
) -> CmdResult<()> {
    app_handle.update_settings(patch)?;
    Ok(())
}
