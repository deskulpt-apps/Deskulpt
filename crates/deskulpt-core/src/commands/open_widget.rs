use deskulpt_common::SerResult;
use tauri::{AppHandle, Runtime, command};

use crate::path::PathExt;

/// Open the widgets directory or a specific widget directory.
///
/// If the widget ID is provided, a specific widget directory will be opened.
/// Otherwise, the widgets directory will be opened.
///
/// ### Errors
///
/// - Widget ID is provided but does not exist in the collection.
/// - Failed to access the widgets directory.
/// - Error opening the directory.
#[command]
#[specta::specta]
pub async fn open_widget<R: Runtime>(
    app_handle: AppHandle<R>,
    id: Option<String>,
) -> SerResult<()> {
    let widgets_dir = app_handle.widgets_dir()?;

    if let Some(id) = id {
        let widget_dir = app_handle.widget_dir(&id)?;
        open::that_detached(widget_dir)?;
    } else {
        open::that_detached(widgets_dir)?;
    };

    Ok(())
}
