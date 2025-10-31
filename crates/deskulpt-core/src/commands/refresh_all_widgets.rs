use std::path::PathBuf;

use anyhow::{Context, Result};
use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use deskulpt_common::window::DeskulptWindow;
use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::bundler::WidgetBundlerBuilder;
use crate::commands::error::cmdbail;
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
pub async fn refresh_all_widgets<R: Runtime>(app_handle: AppHandle<R>) -> CmdResult<()> {
    app_handle.refresh_all_widgets()?;

    let entries: Vec<_> = app_handle
        .get_widget_catalog()
        .0
        .iter()
        .filter_map(|(id, config)| match config {
            Outcome::Ok(config) => Some((id.clone(), config.entry.clone())),
            _ => None,
        })
        .collect();

    let mut errors = vec![];
    let widgets_dir = app_handle.widgets_dir()?;
    for (id, entry) in entries.into_iter() {
        let code = bundle_one(widgets_dir.join(&id), entry)
            .await
            .with_context(|| format!("Failed to bundle widget (id={id})"))
            .into();
        let event = RenderWidgetEvent { id: &id, code };
        if let Err(e) = event.emit_to(&app_handle, DeskulptWindow::Canvas) {
            errors.push(e.context(format!("Failed to render widget (id={id})")));
        }
    }

    if !errors.is_empty() {
        let message = errors
            .into_iter()
            .map(|e| format!("{e}"))
            .collect::<Vec<_>>()
            .join("\n");
        cmdbail!(message);
    }

    Ok(())
}
