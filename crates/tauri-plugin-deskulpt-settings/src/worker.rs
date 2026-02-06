//! Worker for processing settings-related tasks.

use std::pin::Pin;
use std::time::Duration;

use anyhow::Result;
use tauri::{AppHandle, Runtime};
use tokio::sync::mpsc;
use tokio::time::{Instant, Sleep};
use tracing::error;

use crate::SettingsExt;
use crate::model::{CanvasImode, ShortcutAction, Theme};

/// Debounce duration for [`WorkerTask::Persist`].
const PERSIST_DEBOUNCE: Duration = Duration::from_millis(500);

/// Tasks that the worker can process.
#[derive(Debug)]
pub enum WorkerTask {
    /// Persist settings to disk.
    ///
    /// The worker will debounce frequent persist requests within the duration
    /// [`PERSIST_DEBOUNCE`] into a single persist operation to reduce disk I/O.
    /// Note that if the channel is closed unexpectedly, pending persists may be
    /// lost.
    Persist,
    /// Theme has changed.
    ///
    /// The worker will trigger all hooks on theme change.
    ThemeChanged { old: Theme, new: Theme },
    /// Canvas interaction mode has changed.
    ///
    /// The worker will trigger all hooks on canvas interaction mode change.
    CanvasImodeChanged { old: CanvasImode, new: CanvasImode },
    /// Shortcut has changed.
    ///
    /// The worker will trigger all hooks on shortcut change.
    ShortcutChanged {
        action: ShortcutAction,
        old: Option<String>,
        new: Option<String>,
    },
}

/// The worker for processing settings-related tasks.
struct Worker<R: Runtime> {
    /// The Tauri app handle.
    app_handle: AppHandle<R>,
    /// The receiver for incoming tasks.
    rx: mpsc::UnboundedReceiver<WorkerTask>,
    /// Whether a [`WorkerTask::Persist`] is pending.
    persist_pending: bool,
    /// The debounce timer for [`WorkerTask::Persist`].
    persist_debounce: Pin<Box<Sleep>>,
}

impl<R: Runtime> Worker<R> {
    /// Create a new [`Worker`] instance.
    fn new(app_handle: AppHandle<R>, rx: mpsc::UnboundedReceiver<WorkerTask>) -> Self {
        Self {
            app_handle,
            rx,
            persist_pending: false,
            persist_debounce: Box::pin(tokio::time::sleep(PERSIST_DEBOUNCE)),
        }
    }

    /// Run the worker event loop.
    ///
    /// This function will run indefinitely until the worker channel is closed.
    async fn run(mut self) {
        loop {
            tokio::select! {
                _ = &mut self.persist_debounce, if self.persist_pending => {
                    self.on_persist_deadline();
                },
                task = self.rx.recv() => match task {
                    Some(task) => self.handle_task(task),
                    None => break,
                },
            }
        }
    }

    /// Fire the persist operation when the debounce timer elapses.
    fn on_persist_deadline(&mut self) {
        self.persist_pending = false;
        if let Err(e) = self.app_handle.settings().persist() {
            error!("Failed to persist settings: {e:?}");
        }
    }

    /// Handle an incoming [`WorkerTask`].
    fn handle_task(&mut self, task: WorkerTask) {
        match task {
            WorkerTask::Persist => {
                self.persist_pending = true;
                self.persist_debounce
                    .as_mut()
                    .reset(Instant::now() + PERSIST_DEBOUNCE);
            },
            WorkerTask::ThemeChanged { old, new } => {
                self.app_handle.settings().trigger_theme_hooks(&old, &new);
            },
            WorkerTask::CanvasImodeChanged { old, new } => {
                self.app_handle
                    .settings()
                    .trigger_canvas_imode_hooks(&old, &new);
            },
            WorkerTask::ShortcutChanged { action, old, new } => {
                self.app_handle.settings().trigger_shortcut_hooks(
                    &action,
                    old.as_ref(),
                    new.as_ref(),
                );
            },
        }
    }
}

/// Handle for communicating with the worker.
pub struct WorkerHandle(mpsc::UnboundedSender<WorkerTask>);

impl WorkerHandle {
    /// Create a new [`WorkerHandle`] instance.
    ///
    /// This immediately spawns a dedicated worker on Tauri's singleton async
    /// runtime that listens for incoming [`WorkerTask`]s and processes them
    /// asynchronously in order.
    pub fn new<R: Runtime>(app_handle: AppHandle<R>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        tauri::async_runtime::spawn(async move {
            Worker::new(app_handle, rx).run().await;
        });
        Self(tx)
    }

    /// Instruct the worker to process a task.
    ///
    /// This does not block. The task is sent to the worker for asynchronous
    /// processing and does not wait for completion. An error is returned only
    /// if task submission fails, but not if task processing fails.
    pub fn process(&self, task: WorkerTask) -> Result<()> {
        Ok(self.0.send(task)?)
    }
}
