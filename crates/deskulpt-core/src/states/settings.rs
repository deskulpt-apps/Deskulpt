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

#[derive(Default)]
struct SettingsHooks {
    on_theme_change: Vec<OnThemeChange>,
    on_shortcut_change: Vec<OnShortcutChange>,
}

#[derive(Debug)]
enum SettingsUpdateJob {
    Theme {
        old: Theme,
        new: Theme,
    },
    Shortcut {
        key: ShortcutKey,
        old: Option<String>,
        new: Option<String>,
    },
}

/// Managed state for the settings.
struct SettingsState {
    inner: RwLock<Settings>,
    hooks: RwLock<SettingsHooks>,
    job_sender: mpsc::UnboundedSender<SettingsUpdateJob>,
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

        let (tx, mut rx) = mpsc::unbounded_channel::<SettingsUpdateJob>();
        let app_handle = self.app_handle().clone();
        tauri::async_runtime::spawn(async move {
            while let Some(job) = rx.recv().await {
                match job {
                    SettingsUpdateJob::Theme { old, new } => {
                        let state = app_handle.state::<SettingsState>();
                        let hooks = state.hooks.read().unwrap();
                        for hook in &hooks.on_theme_change {
                            hook(&old, &new);
                        }
                    },
                    SettingsUpdateJob::Shortcut { key, old, new } => {
                        let state = app_handle.state::<SettingsState>();
                        let hooks = state.hooks.read().unwrap();
                        for hook in &hooks.on_shortcut_change {
                            hook(&key, old.as_ref(), new.as_ref());
                        }
                    },
                }
            }
        });

        self.manage(SettingsState {
            inner: RwLock::new(settings),
            hooks: Default::default(),
            job_sender: tx,
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

    fn on_theme_change<F>(&self, hook: F)
    where
        F: Fn(&Theme, &Theme) + Send + Sync + 'static,
    {
        let state = self.state::<SettingsState>();
        let mut hooks = state.hooks.write().unwrap();
        hooks.on_theme_change.push(Box::new(hook));
    }

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

        let mut jobs = vec![];
        let mut dirty = false;

        if let Some(theme) = patch.theme
            && settings.theme != theme
        {
            let old_theme = std::mem::replace(&mut settings.theme, theme.clone());
            jobs.push(SettingsUpdateJob::Theme {
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
                    jobs.push(SettingsUpdateJob::Shortcut {
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
        for job in jobs {
            if let Err(e) = state.job_sender.send(job) {
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
