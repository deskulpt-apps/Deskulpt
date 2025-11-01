//! Bundler for Deskulpt widgets.

mod alias;

use std::path::PathBuf;
use std::sync::Arc;

use alias::AliasPlugin;
use anyhow::{Result, anyhow, bail};
use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use deskulpt_common::window::DeskulptWindow;
use deskulpt_core::path::PathExt;
use either::Either;
use rolldown::{
    BundlerOptions, BundlerTransformOptions, JsxOptions, OutputFormat, Platform, RawMinifyOptions,
};
use rolldown_common::Output;
use tauri::{AppHandle, Runtime};
use tokio::sync::mpsc::UnboundedSender;

use crate::Widgets;
use crate::events::RenderEvent;

/// Builder for the Deskulpt widget bundler.
struct BundlerBuilder {
    /// Absolute path to the widget directory.
    root: PathBuf,
    /// Entry file relative to the widget directory.
    entry: String,
}

impl BundlerBuilder {
    /// Create a new widget bundler builder instance.
    pub fn new(root: PathBuf, entry: String) -> Self {
        Self { root, entry }
    }

    /// Build the Deskulpt widget bundler.
    pub fn build(self) -> Result<Bundler> {
        const JSX_RUNTIME_URL: &str = "__DESKULPT_BASE_URL__/gen/jsx-runtime.js";
        const RAW_APIS_URL: &str = "__DESKULPT_BASE_URL__/gen/raw-apis.js";
        const REACT_URL: &str = "__DESKULPT_BASE_URL__/gen/react.js";
        const UI_URL: &str = "__DESKULPT_BASE_URL__/gen/ui.js";
        const APIS_BLOB_URL: &str = "__DESKULPT_APIS_BLOB_URL__";

        let bundler_options = BundlerOptions {
            input: Some(vec![self.entry.into()]),
            cwd: Some(self.root),
            format: Some(OutputFormat::Esm),
            platform: Some(Platform::Browser),
            minify: Some(RawMinifyOptions::Bool(true)),
            // Use automatic runtime for JSX transforms, which will refer to
            // `@deskulpt-test/emotion/jsx-runtime`
            transform: Some(BundlerTransformOptions {
                jsx: Some(Either::Right(JsxOptions {
                    runtime: Some("automatic".to_string()),
                    import_source: Some("@deskulpt-test/emotion".to_string()),
                    ..Default::default()
                })),
                ..Default::default()
            }),
            // Externalize default dependencies available at runtime
            external: Some(
                vec![
                    JSX_RUNTIME_URL.to_string(),
                    RAW_APIS_URL.to_string(),
                    REACT_URL.to_string(),
                    UI_URL.to_string(),
                    APIS_BLOB_URL.to_string(),
                ]
                .into(),
            ),
            ..Default::default()
        };

        // Alias the default dependencies to URLs resolvable at runtime
        let alias_plugin = AliasPlugin(
            [
                (
                    "@deskulpt-test/emotion/jsx-runtime".to_string(),
                    JSX_RUNTIME_URL.to_string(),
                ),
                (
                    "@deskulpt-test/raw-apis".to_string(),
                    RAW_APIS_URL.to_string(),
                ),
                ("@deskulpt-test/react".to_string(), REACT_URL.to_string()),
                ("@deskulpt-test/ui".to_string(), UI_URL.to_string()),
                ("@deskulpt-test/apis".to_string(), APIS_BLOB_URL.to_string()),
            ]
            .into(),
        );

        let bundler =
            rolldown::Bundler::with_plugins(bundler_options, vec![Arc::new(alias_plugin)])?;
        Ok(Bundler(bundler))
    }
}

/// The Deskulpt widget bundler.
struct Bundler(rolldown::Bundler);

impl Bundler {
    /// Bundle the widget into a single ESM code string.
    pub async fn bundle(&mut self) -> Result<String> {
        let result = self.0.generate().await.map_err(|e| {
            anyhow!(
                e.into_vec()
                    .iter()
                    .map(|diagnostic| diagnostic.to_diagnostic().to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        })?;

        // We have supplied a single entry file, so we expect a single output
        // bundle; this can be broken if widget code contains e.g. dynamic
        // imports, which we do not allow
        if result.assets.len() != 1 {
            bail!(
                "Expected 1 bundled output, found {}; ensure that widget code does not contain \
                 e.g. dynamic imports that may result in extra chunks",
                result.assets.len()
            );
        }

        let output = &result.assets[0];
        let code = match output {
            Output::Asset(asset) => asset.source.clone().try_into_string()?,
            Output::Chunk(chunk) => chunk.code.clone(),
        };
        Ok(code)
    }
}

/// Task to render a specific widget.
struct RenderWidgetTask {
    /// The widget ID.
    id: String,
    /// The entry file relative to the widget directory.
    entry: String,
}

pub struct RenderWorkerHandle(UnboundedSender<RenderWidgetTask>);

impl RenderWorkerHandle {
    pub fn new<R: Runtime>(app_handle: AppHandle<R>) -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<RenderWidgetTask>();

        tauri::async_runtime::spawn(async move {
            while let Some(task) = rx.recv().await {
                let code = async {
                    let widget_dir = app_handle.widgets_dir()?.join(&task.id);
                    let mut bundler = BundlerBuilder::new(widget_dir, task.entry).build()?;
                    let code = bundler.bundle().await?;
                    Ok::<_, anyhow::Error>(code)
                }
                .await
                .into();
                let event = RenderEvent { id: &task.id, code };
                if let Err(e) = event.emit_to(&app_handle, DeskulptWindow::Canvas) {
                    eprintln!("Failed to emit RenderEvent for widget {}: {e:?}", task.id);
                };
            }
        });

        Self(tx)
    }

    fn send(&self, task: RenderWidgetTask) -> Result<()> {
        Ok(self.0.send(task)?)
    }
}

impl<R: Runtime> Widgets<R> {
    /// Render a specific widget by its ID.
    ///
    /// If the widget does not exist in the catalog, an error is returned.
    /// Otherwise, a render task is sent to the render worker. If this fails,
    /// an error is returned as well. This function does not wait for the
    /// rendering to complete.
    pub fn render(&self, id: String) -> Result<()> {
        let catalog = self.catalog.read().unwrap();
        let config = catalog
            .0
            .get(&id)
            .ok_or_else(|| anyhow!("Widget {id} does not exist in the catalog"))?;

        if let Outcome::Ok(config) = config {
            self.render_handle.send(RenderWidgetTask {
                id,
                entry: config.entry.clone(),
            })?;
        }
        Ok(())
    }

    /// Render all widgets in the catalog.
    ///
    /// This function sends render tasks for all widgets in the catalog to the
    /// render worker. If any task fails to be sent, an error is returned
    /// containing all individual errors. This function does not wait for the
    /// rendering to complete.
    pub fn render_all(&self) -> Result<()> {
        let catalog = self.catalog.read().unwrap();

        let mut errors = vec![];
        for (id, config) in catalog.0.iter() {
            if let Outcome::Ok(config) = config {
                if let Err(e) = self.render_handle.send(RenderWidgetTask {
                    id: id.clone(),
                    entry: config.entry.clone(),
                }) {
                    errors.push(e.context(format!("Failed to send render task for widget {id}")));
                }
            }
        }

        if !errors.is_empty() {
            let message = errors
                .into_iter()
                .map(|e| format!("{e:?}"))
                .collect::<Vec<_>>()
                .join("\n");
            bail!(message);
        }

        Ok(())
    }
}
