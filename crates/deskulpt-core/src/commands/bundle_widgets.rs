use std::collections::HashMap;

use anyhow::Context;
use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use deskulpt_common::window::DeskulptWindow;
use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::bundler::WidgetBundlerBuilder;
use crate::events::RenderWidgetsEvent;
use crate::path::PathExt;
use crate::states::WidgetCatalogStateExt;

/// Bundle widgets.
///
/// This command bundles the specified widgets that exist in the catalog. If
/// `ids` is not provided, all widgets in the catalog are bundled. Failure to
/// bundle an individual widget does not prevent other widgets from being
/// bundled. Instead, the outcome of each bundling operation is collected and
/// sent to the canvas window via the [`RenderWidgetsEvent`].
///
/// ### Errors
///
/// - Error accessing the widgets directory.
/// - Error emitting the [`RenderWidgetsEvent`].
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

    let futs = widgets.into_iter().map(|(id, entry)| async move {
        let report = match WidgetBundlerBuilder::new(widgets_dir.join(&id), entry)
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
        (id, report)
    });

    let reports = futures::future::join_all(futs)
        .await
        .into_iter()
        .collect::<HashMap<_, _>>();

    RenderWidgetsEvent(&reports).emit_to(&app_handle, DeskulptWindow::Canvas)?;
    Ok(())
}
