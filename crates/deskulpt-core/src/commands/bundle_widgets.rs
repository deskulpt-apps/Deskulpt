use anyhow::Context;
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

/// Bundle widgets.
///
/// This command bundles the specified widgets that exist in the catalog. If
/// `ids` is not provided, all widgets in the catalog are bundled. Failure to
/// bundle a widget does not result in an error, but is reported back to the
/// canvas window via the [`RenderWidgetEvent`]. Moreover, failure to emit a
/// single [`RenderWidgetEvent`] does not prevent other widgets from being
/// processed; instead, errors are collected and returned as a single error at
/// the end, if any.
///
/// ### Errors
///
/// - Error accessing the widgets directory.
/// - Error emitting [`RenderWidgetEvent`] for one or more widgets.
#[command]
#[specta::specta]
pub async fn bundle_widgets<R: Runtime>(
    app_handle: AppHandle<R>,
    ids: Option<Vec<String>>,
) -> CmdResult<()> {
    let widgets_dir = app_handle.widgets_dir()?;

    let widgets: Vec<_> = {
        let catalog = app_handle.get_widget_catalog();
        match ids {
            Some(ids) => ids
                .into_iter()
                .filter_map(|id| match catalog.0.get(&id) {
                    Some(Outcome::Ok(config)) => Some((id, config.entry.clone())),
                    _ => None,
                })
                .collect(),
            None => catalog
                .0
                .iter()
                .filter_map(|(id, config)| match config {
                    Outcome::Ok(config) => Some((id.clone(), config.entry.clone())),
                    _ => None,
                })
                .collect(),
        }
    };

    if widgets.is_empty() {
        return Ok(());
    }

    let start = std::time::Instant::now();
    let mut errors = vec![];
    let num_widgets = widgets.len();
    for (id, entry) in widgets.into_iter() {
        let code = match WidgetBundlerBuilder::new(widgets_dir.join(&id), entry)
            .build()
            .context("Failed to build widget bundler")
        {
            Ok(mut bundler) => bundler
                .bundle()
                .await
                .with_context(|| format!("Failed to bundle widget (id={id})"))
                .map_or_else(|e| Outcome::Err(format!("{e:?}")), Outcome::Ok),
            Err(e) => Outcome::Err(format!("{e:?}")),
        };

        let event = RenderWidgetEvent { id: &id, code };
        if let Err(e) = event.emit_to(&app_handle, DeskulptWindow::Canvas) {
            errors.push(e.context(format!("Failed to render widget (id={id})")));
        }
    }
    let duration = std::time::Instant::now() - start;
    println!("Bundled {num_widgets} widgets in {duration:?}");

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
