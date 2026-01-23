//! Render worker for Deskulpt widgets.

use anyhow::Result;
use deskulpt_common::event::Event;
use deskulpt_common::window::DeskulptWindow;
use tauri::{AppHandle, Runtime};
use tokio::sync::mpsc;
use tracing::error;

use crate::WidgetsExt;
use crate::events::RenderEvent;
use crate::render::bundler::Bundler;

/// Tasks that the render worker can process.
#[derive(Debug)]
pub enum RenderWorkerTask {
    /// Bundle and render a widget.
    ///
    /// The worker will use [`Bundler`] to bundle the specified widget at the
    /// specified entry file. Upon completion, a [`RenderEvent`] will be emitted
    /// to the canvas window with the bundling result, whether success or
    /// failure.
    Render {
        /// The widget ID.
        id: String,
        /// The entry file path relative to the root of the widget.
        entry: String,
    },
}

/// The main render worker loop.
async fn render_worker<R: Runtime>(
    app_handle: AppHandle<R>,
    mut rx: mpsc::UnboundedReceiver<RenderWorkerTask>,
) {
    while let Some(task) = rx.recv().await {
        match task {
            RenderWorkerTask::Render { id, entry } => {
                let report = async {
                    let widget_dir = app_handle.widgets().dir().join(&id);
                    let code = Bundler::new(widget_dir, entry)?.bundle().await?;
                    Ok::<_, anyhow::Error>(code)
                }
                .await
                .into();
                let event = RenderEvent { id: &id, report };
                if let Err(e) = event.emit_to(&app_handle, DeskulptWindow::Canvas) {
                    error!("Failed to emit RenderEvent for widget {id}: {e:?}");
                };
            },
        }
    }
}

/// Handle for communicating with the render worker.
pub struct RenderWorkerHandle(mpsc::UnboundedSender<RenderWorkerTask>);

impl RenderWorkerHandle {
    /// Create a new [`RenderWorkerHandle`] instance.
    ///
    /// This immediately spawns a dedicated render worker on Tauri's singleton
    /// async runtime that listens for incoming [`RenderWorkerTask`]s and
    /// processes them asynchronously in order.
    pub fn new<R: Runtime>(app_handle: AppHandle<R>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        tauri::async_runtime::spawn(async move {
            render_worker(app_handle, rx).await;
        });
        Self(tx)
    }

    /// Instruct the render worker to process a task.
    ///
    /// This does not block. The task is sent to the render worker for
    /// asynchronous processing and does not wait for completion. An error is
    /// returned if task submission fails, but not task processing fails.
    pub fn process(&self, task: RenderWorkerTask) -> Result<()> {
        Ok(self.0.send(task)?)
    }
}
