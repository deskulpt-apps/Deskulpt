use deskulpt_common::SerResult;
use serde::Deserialize;
use tauri::{AppHandle, Runtime, command};

use crate::path::PathExt;

/// The target to open.
#[derive(Debug, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum OpenTarget {
    /// The widgets base directory.
    Widgets,
    /// A specific widget directory by its ID.
    Widget(String),
    /// The persisted settings file.
    Settings,
    /// The logs directory.
    Logs,
}

/// Open a specified target with the system's default application.
///
/// See [`OpenTarget`] for more details.
///
/// ### Errors
///
/// - Error accessing the specified target.
/// - Error opening the target.
#[command]
#[specta::specta]
pub async fn open<R: Runtime>(app_handle: AppHandle<R>, target: OpenTarget) -> SerResult<()> {
    let path = match target {
        OpenTarget::Widgets => app_handle.widgets_dir()?,
        OpenTarget::Widget(id) => &app_handle.widget_dir(&id)?,
        OpenTarget::Settings => &app_handle.persist_dir()?.join("settings.json"),
        OpenTarget::Logs => app_handle.logs_dir()?,
    };

    open::that_detached(path)?;
    Ok(())
}
