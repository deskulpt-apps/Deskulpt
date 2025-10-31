use std::path::PathBuf;

use anyhow::{Context, Result};
use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use deskulpt_common::window::DeskulptWindow;
use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::bundler::WidgetBundlerBuilder;
use crate::events::RenderWidgetEvent;
use crate::path::PathExt;
use crate::states::WidgetCatalogStateExt;

async fn bundle_one(root: PathBuf, entry: String) -> Result<String> {
    let mut bundler = WidgetBundlerBuilder::new(root, entry).build()?;
    let code = bundler.bundle().await?;
    Ok(code)
}

/// TODO(Charlie-XIAO)
#[command]
#[specta::specta]
pub async fn refresh_widget<R: Runtime>(app_handle: AppHandle<R>, id: String) -> CmdResult<()> {
    app_handle.refresh_widget(&id)?;

    let entry = if let Some(Outcome::Ok(config)) = app_handle.get_widget_catalog().0.get(&id) {
        Some(config.entry.clone())
    } else {
        None
    };

    if let Some(entry) = entry {
        let widget_dir = app_handle.widgets_dir()?.join(&id);
        let code = bundle_one(widget_dir, entry)
            .await
            .with_context(|| format!("Failed to bundle widget (id={id})"))
            .into();
        let event = RenderWidgetEvent { id: &id, code };
        event.emit_to(&app_handle, DeskulptWindow::Canvas)?;
    }

    Ok(())
}
