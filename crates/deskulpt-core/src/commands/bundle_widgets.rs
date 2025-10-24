use std::collections::HashMap;

use anyhow::Context;
use deskulpt_common::outcome::Outcome;
use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::bundler::WidgetBundlerBuilder;
use crate::events::RenderWidgetsEvent;
use crate::path::PathExt;
use crate::states::{InitialRenderStateExt, WidgetCatalogStateExt};

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
                .filter_map(|id| catalog.0.get(&id).cloned().map(|config| (id, config)))
                .collect(),
            None => catalog
                .0
                .iter()
                .map(|(id, config)| (id.clone(), config.clone()))
                .collect(),
        }
    };

    if widgets.is_empty() {
        return Ok(());
    }

    let futs = widgets.into_iter().map(|(id, config)| async move {
        match config {
            Outcome::Ok(config) => {
                let report =
                    match WidgetBundlerBuilder::new(widgets_dir.join(&id), config.entry.clone())
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
            },
            Outcome::Err(e) => (id, Outcome::Err(e.clone())),
        }
    });

    let reports = futures::future::join_all(futs)
        .await
        .into_iter()
        .collect::<HashMap<_, _>>();

    app_handle.emit_on_render_ready(RenderWidgetsEvent(reports))?;
    Ok(())
}
