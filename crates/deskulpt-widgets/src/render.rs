//! Backend preprocessing for Deskulpt widgets rendering.

use anyhow::Result;
use deskulpt_common::event::Event;
use deskulpt_common::window::DeskulptWindow;
use deskulpt_core::path::PathExt;
use tauri::{AppHandle, Runtime};
use tokio::sync::mpsc;
use tracing::{Instrument, Span, info_span};

use crate::events::RenderEvent;
use crate::render::bundler::Bundler;

mod alias_plugin;
mod bundler;
mod worker;

/// A task for the render worker.
pub enum RenderWorkerTask {
    /// Bundle and render a widget.
    Render {
        /// The widget ID.
        id: String,
        /// The entry file path, relative to the widget directory.
        entry: String,
        /// The tracing span associated with this render task.
        span: Span,
    },
}

/// Process a [`RenderWorkerTask::Render`] task.
///
/// This bundles the specified widget and emits a [`RenderEvent`] to the canvas
/// window with the bundling result.
async fn process_render_task<R: Runtime>(app_handle: &AppHandle<R>, id: String, entry: String) {
    let entry_for_span = entry.clone();
    let id_for_task = id.clone();
    let span = info_span!(
        "widget.render_task",
        widget_id = %id,
        entry = %entry_for_span,
        task_kind = "render",
        status = tracing::field::Empty,
    );
    let report = async move {
        let widget_dir = app_handle.widgets_dir()?.join(&id_for_task);
        let code = Bundler::new(widget_dir, entry)?.bundle().await?;
        Ok::<_, anyhow::Error>(code)
    }
    .instrument(span)
    .await
    .into();
    let event = RenderEvent { id: &id, report };
    if let Err(e) = event.emit_to(app_handle, DeskulptWindow::Canvas) {
        tracing::error!(
            widget_id = %id,
            operation = "emit_render_event",
            status = "error",
            error_kind = ?e,
            "Failed to emit RenderEvent to canvas",
        );
    };
}

/// Handle for communicating with the render worker.
pub struct RenderWorkerHandle(mpsc::UnboundedSender<RenderWorkerTask>);

impl RenderWorkerHandle {
    /// Create a new [`RenderWorkerHandle`] instance.
    ///
    /// This immediately spawns the render worker on the async runtime that
    /// listens for incoming [`RenderWorkerTask`]s and processes them.
    pub fn new<R: Runtime>(app_handle: AppHandle<R>) -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel::<RenderWorkerTask>();

        tauri::async_runtime::spawn(async move {
            while let Some(task) = rx.recv().await {
                match task {
                    RenderWorkerTask::Render { id, entry, span } => {
                        async {
                            process_render_task(&app_handle, id, entry).await;
                        }
                        .instrument(span)
                        .await;
                    },
                }
            }
        });

        Self(tx)
    }

    /// Process a [`RenderWorkerTask`].
    ///
    /// This does not block. The task is sent to the render worker for
    /// asynchronous processing and does not wait for completion.
    pub fn process(&self, task: RenderWorkerTask) -> Result<()> {
        Ok(self.0.send(task)?)
    }
}
