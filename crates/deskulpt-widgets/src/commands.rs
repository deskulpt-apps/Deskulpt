//! Tauri commands.

use deskulpt_common::SerResult;
use tauri::{AppHandle, Runtime, WebviewWindow};

use crate::WidgetsExt;

/// Wrapper of [`crate::Widgets::refresh`].
#[tauri::command]
#[specta::specta]
pub async fn refresh<R: Runtime>(app_handle: AppHandle<R>, id: String) -> SerResult<()> {
    app_handle.widgets().refresh(&id)?;
    Ok(())
}

/// Wrapper of [`crate::Widgets::refresh_all`].
#[tauri::command]
#[specta::specta]
pub async fn refresh_all<R: Runtime>(app_handle: AppHandle<R>) -> SerResult<()> {
    app_handle.widgets().refresh_all()?;
    Ok(())
}

/// Wrapper of [`crate::Widgets::complete_setup`].
#[tauri::command]
#[specta::specta]
pub async fn complete_setup<R: Runtime>(
    app_handle: AppHandle<R>,
    window: WebviewWindow<R>,
) -> SerResult<()> {
    app_handle.widgets().complete_setup(window)?;
    Ok(())
}
