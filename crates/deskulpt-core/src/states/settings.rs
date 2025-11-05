//! State management for the settings.

use std::sync::{RwLock, RwLockReadGuard};

use anyhow::{Result, bail};
use deskulpt_common::event::Event;
use tauri::{App, AppHandle, Emitter, Manager, Runtime};
use tokio::sync::mpsc;

use crate::events::UpdateSettingsEvent;
use crate::path::PathExt;
use crate::settings::{Settings, SettingsPatch, ShortcutKey, Theme};

type OnThemeChange = Box<dyn Fn(&Theme, &Theme) + Send + Sync>;
type OnShortcutChange = Box<dyn Fn(&ShortcutKey, Option<&String>, Option<&String>) + Send + Sync>;

/// The collection of hooks on settings changes.
#[derive(Default)]
struct SettingsHooks {
    /// The collection of hooks on theme changes.
    ///
    /// See [`SettingsStateExt::on_theme_change`] for more details.
    on_theme_change: Vec<OnThemeChange>,
    /// The collection of hooks on shortcut changes.
    ///
    /// See [`SettingsStateExt::on_shortcut_change`] for more details.
    on_shortcut_change: Vec<OnShortcutChange>,
}

/// A task for the settings worker.
#[derive(Debug)]
enum SettingsWorkerTask {
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

/// Managed state for Deskulpt settings.
struct SettingsState {
    /// The settings.
    inner: RwLock<Settings>,
    /// The collection of hooks on settings change.
    hooks: RwLock<SettingsHooks>,
    /// The handle for the settings worker.
    worker: SettingsWorkerHandle,
}

/// Handle for communicating with the settings worker.
struct SettingsWorkerHandle(mpsc::UnboundedSender<SettingsWorkerTask>);

impl SettingsWorkerHandle {
    /// Create a new [`SettingsWorkerHandle`] instance.
    ///
    /// This immediately spawns the settings worker on the async runtime that
    /// listens for incoming [`SettingsWorkerTask`]s and processes them.
    fn new<R: Runtime>(app_handle: AppHandle<R>) -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel::<SettingsWorkerTask>();

        tauri::async_runtime::spawn(async move {
            while let Some(task) = rx.recv().await {
                match task {
                    SettingsWorkerTask::Theme { old, new } => {
                        let state = app_handle.state::<SettingsState>();
                        let hooks = state.hooks.read().unwrap();
                        for hook in &hooks.on_theme_change {
                            hook(&old, &new);
                        }
                    },
                    SettingsWorkerTask::Shortcut { key, old, new } => {
                        let state = app_handle.state::<SettingsState>();
                        let hooks = state.hooks.read().unwrap();
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
    fn process(&self, task: SettingsWorkerTask) -> Result<()> {
        Ok(self.0.send(task)?)
    }
}

/// Extension trait for operations on the settings state.
pub trait SettingsStateExt<R: Runtime>: Manager<R> + Emitter<R> + PathExt<R> {
    /// Initialize state management for the settings.
    ///
    /// This will load the settings from the persistence directory and
    /// initialize the shortcuts. If any step fails, it will fall back to a
    /// state that preserves as much persisted data as possible.
    fn manage_settings(&self) {
        let settings = self
            .persist_dir()
            .and_then(Settings::load)
            .unwrap_or_else(|e| {
                eprintln!("Failed to load settings: {e}");
                Settings::default()
            });

        let worker = SettingsWorkerHandle::new(self.app_handle().clone());

        self.manage(SettingsState {
            inner: RwLock::new(settings),
            hooks: Default::default(),
            worker,
        });
    }

    /// Get an immutable reference to the settings.
    ///
    /// The returned reference is behind a lock guard, which should be dropped
    /// as soon as possible to minimize critical section.
    fn get_settings(&self) -> RwLockReadGuard<'_, Settings> {
        let state = self.state::<SettingsState>().inner();
        state.inner.read().unwrap()
    }

    /// Register a hook that will be triggered on theme change.
    ///
    /// The two arguments are respectively the old and new themes.
    fn on_theme_change<F>(&self, hook: F)
    where
        F: Fn(&Theme, &Theme) + Send + Sync + 'static,
    {
        let state = self.state::<SettingsState>();
        let mut hooks = state.hooks.write().unwrap();
        hooks.on_theme_change.push(Box::new(hook));
    }

    /// Register a hook that will be triggered on shortcut change.
    ///
    /// The first argument is the shortcut key. The second and third arguments
    /// are respectively the old and new shortcuts. `None` means that no
    /// shortcut was/is assigned for that key.
    fn on_shortcut_change<F>(&self, hook: F)
    where
        F: Fn(&ShortcutKey, Option<&String>, Option<&String>) + Send + Sync + 'static,
    {
        let state = self.state::<SettingsState>();
        let mut hooks = state.hooks.write().unwrap();
        hooks.on_shortcut_change.push(Box::new(hook));
    }

    /// Update the settings.
    ///
    /// TODO
    fn update_settings<F>(&self, update: F) -> Result<()>
    where
        F: FnOnce(&Settings) -> SettingsPatch,
        Self: Sized,
    {
        let state = self.state::<SettingsState>();
        let mut settings = state.inner.write().unwrap();
        let patch = update(&settings);

        let mut tasks = vec![];
        let mut dirty = false;

        if let Some(theme) = patch.theme
            && settings.theme != theme
        {
            let old_theme = std::mem::replace(&mut settings.theme, theme.clone());
            tasks.push(SettingsWorkerTask::Theme {
                old: old_theme,
                new: theme,
            });
            dirty = true;
        }

        if let Some(shortcuts) = patch.shortcuts {
            for (key, shortcut) in shortcuts {
                let old_shortcut = match &shortcut {
                    Some(shortcut) => settings.shortcuts.insert(key.clone(), shortcut.clone()),
                    None => settings.shortcuts.remove(&key),
                };
                if old_shortcut != shortcut {
                    tasks.push(SettingsWorkerTask::Shortcut {
                        key,
                        old: old_shortcut,
                        new: shortcut,
                    });
                    dirty = true;
                }
            }
        }

        if let Some(widgets) = patch.widgets {
            for (id, patch) in widgets {
                match patch {
                    Some(patch) => {
                        let widget = settings.widgets.entry(id).or_insert_with(|| {
                            dirty = true;
                            Default::default()
                        });
                        dirty |= widget.apply_patch(patch);
                    },
                    None => dirty |= settings.widgets.remove(&id).is_some(),
                }
            }
        }

        if dirty {
            UpdateSettingsEvent(&settings).emit(self)?;
        }

        let mut errors = vec![];
        for task in tasks {
            if let Err(e) = state.worker.process(task) {
                errors.push(e);
            }
        }
        if !errors.is_empty() {
            let message = errors
                .into_iter()
                .map(|e| format!("{e:?}"))
                .collect::<Vec<_>>()
                .join("\n\n");
            bail!("One or more changes failed to be submitted\n\n{message}");
        }

        Ok(())
    }
}

impl<R: Runtime> SettingsStateExt<R> for App<R> {}
impl<R: Runtime> SettingsStateExt<R> for AppHandle<R> {}
