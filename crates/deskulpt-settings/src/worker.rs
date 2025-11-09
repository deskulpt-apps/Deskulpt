use anyhow::Result;
use tauri::{AppHandle, Runtime};
use tokio::sync::mpsc;

use crate::SettingsExt;
use crate::settings::{ShortcutKey, Theme};

/// A task for the settings worker.
#[derive(Debug)]
pub enum WorkerTask {
    /// Theme change.
    ///
    /// The worker will trigger all hooks on theme change.
    Theme { old: Theme, new: Theme },
    /// Shortcut change.
    ///
    /// The worker will trigger all hooks on shortcut change.
    Shortcut {
        key: ShortcutKey,
        old: Option<String>,
        new: Option<String>,
    },
}

/// Handle for communicating with the settings worker.
pub struct WorkerHandle(mpsc::UnboundedSender<WorkerTask>);

impl WorkerHandle {
    /// Create a new [`SettingsWorkerHandle`] instance.
    ///
    /// This immediately spawns the settings worker on the async runtime that
    /// listens for incoming [`SettingsWorkerTask`]s and processes them.
    pub fn new<R: Runtime>(app_handle: AppHandle<R>) -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel();

        tauri::async_runtime::spawn(async move {
            while let Some(task) = rx.recv().await {
                match task {
                    WorkerTask::Theme { old, new } => {
                        let hooks = app_handle.settings().hooks.read().unwrap();
                        for hook in &hooks.on_theme_change {
                            hook(&old, &new);
                        }
                    },
                    WorkerTask::Shortcut { key, old, new } => {
                        let hooks = app_handle.settings().hooks.read().unwrap();
                        for hook in &hooks.on_shortcut_change {
                            hook(&key, old.as_ref(), new.as_ref());
                        }
                    },
                }
            }
        });

        Self(tx)
    }

    /// Process a [`SettingsWorkerTask`].
    ///
    /// This does not block. The task is sent to the settings worker for
    /// asynchronous processing and does not wait for completion.
    pub fn process(&self, task: WorkerTask) -> Result<()> {
        Ok(self.0.send(task)?)
    }
}
