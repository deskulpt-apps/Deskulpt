//! Tauri commands.

use deskulpt_common::SerResult;
use tauri::{AppHandle, Runtime, WebviewWindow};

use crate::WidgetsExt;

/// Wrapper of [`crate::Widgets::bundle`].
#[tauri::command]
#[specta::specta]
pub async fn bundle<R: Runtime>(app_handle: AppHandle<R>, id: Option<String>) -> SerResult<()> {
    app_handle.widgets().bundle(id)?;
    Ok(())
}

/// Wrapper of [`crate::Widgets::rescan`].
#[tauri::command]
#[specta::specta]
pub async fn rescan<R: Runtime>(app_handle: AppHandle<R>) -> SerResult<()> {
    app_handle.widgets().rescan()?;
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
