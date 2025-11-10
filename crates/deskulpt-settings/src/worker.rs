//! Worker for processing settings-related tasks.

use anyhow::Result;
use tauri::{AppHandle, Runtime};
use tokio::sync::mpsc::{self, UnboundedReceiver};

use crate::SettingsExt;
use crate::settings::{ShortcutAction, Theme};

/// Tasks that the worker can process.
#[derive(Debug)]
pub enum WorkerTask {
    /// Theme has changed.
    ///
    /// The worker will trigger all hooks on theme change.
    ThemeChanged { old: Theme, new: Theme },
    /// Shortcut has changed.
    ///
    /// The worker will trigger all hooks on shortcut change.
    ShortcutChanged {
        action: ShortcutAction,
        old: Option<String>,
        new: Option<String>,
    },
}

/// Handle for communicating with the worker.
pub struct WorkerHandle(mpsc::UnboundedSender<WorkerTask>);

/// The main worker loop.
async fn worker<R: Runtime>(app_handle: AppHandle<R>, mut rx: UnboundedReceiver<WorkerTask>) {
    while let Some(task) = rx.recv().await {
        match task {
            WorkerTask::ThemeChanged { old, new } => {
                app_handle.settings().trigger_theme_hooks(&old, &new);
            },
            WorkerTask::ShortcutChanged { action, old, new } => {
                app_handle
                    .settings()
                    .trigger_shortcut_hooks(&action, old.as_ref(), new.as_ref());
            },
        }
    }
}

impl WorkerHandle {
    /// Create a new [`WorkerHandle`] instance.
    ///
    /// This immediately spawns a dedicated worker on Tauri's singleton async
    /// runtime that listens for incoming [`WorkerTask`]s and processes them
    /// asynchronously in order.
    pub fn new<R: Runtime>(app_handle: AppHandle<R>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        tauri::async_runtime::spawn(async move {
            worker(app_handle, rx).await;
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
