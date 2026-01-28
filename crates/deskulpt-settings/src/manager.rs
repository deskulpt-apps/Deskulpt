//! Deskulpt settings manager and its APIs.

use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow, bail};
use deskulpt_common::event::Event;
use parking_lot::{RwLock, RwLockReadGuard};
use tauri::{AppHandle, Manager, Runtime};
use tracing::error;
use url::Url;

use crate::CanvasImode;
use crate::events::UpdateEvent;
use crate::settings::{Settings, SettingsPatch, ShortcutAction, Theme};
use crate::worker::{WorkerHandle, WorkerTask};

#[doc(hidden)]
type OnThemeChange = Box<dyn Fn(&Theme, &Theme) + Send + Sync>;

#[doc(hidden)]
type OnCanvasImodeChange = Box<dyn Fn(&CanvasImode, &CanvasImode) + Send + Sync>;

#[doc(hidden)]
type OnShortcutChange =
    Box<dyn Fn(&ShortcutAction, Option<&String>, Option<&String>) + Send + Sync>;

/// The collection of hooks on settings change.
#[derive(Default)]
struct SettingsHooks {
    /// Hooks triggered on theme change.
    ///
    /// See [`SettingsManager::on_theme_change`] for registration.
    on_theme_change: Vec<OnThemeChange>,
    /// Hooks triggered on canvas interaction mode change.
    ///
    /// See [`SettingsManager::on_canvas_imode_change`] for registration.
    on_canvas_imode_change: Vec<OnCanvasImodeChange>,
    /// Hooks triggered on shortcut change.
    ///
    /// See [`SettingsManager::on_shortcut_change`] for registration.
    on_shortcut_change: Vec<OnShortcutChange>,
}

/// Manager for Deskulpt settings.
pub struct SettingsManager<R: Runtime> {
    /// The Tauri app handle.
    app_handle: AppHandle<R>,
    /// The path where settings are persisted.
    persist_path: PathBuf,
    /// The URL to the settings schema file.
    schema_url: String,
    /// The Deskulpt settings.
    settings: RwLock<Settings>,
    /// The handle for the worker.
    worker: WorkerHandle,
    /// The collection of hooks on settings change.
    hooks: RwLock<SettingsHooks>,
}

impl<R: Runtime> SettingsManager<R> {
    /// Initialize the [`SettingsManager`].
    ///
    /// The settings are loaded from disk. If loading fails (which means
    /// corrupted settings), default settings are used. A worker is started
    /// immediately.
    pub fn new(app_handle: AppHandle<R>) -> Result<Self> {
        let persist_path = app_handle
            .path()
            .app_local_data_dir()?
            .join("settings.json");

        let settings = Settings::load(&persist_path).unwrap_or_else(|e| {
            error!("Failed to load settings: {e:?}");
            Default::default()
        });

        let schema_path = app_handle
            .path()
            .resource_dir()?
            .join("resources")
            .join("schema")
            .join("settings.json");
        let schema_url = Url::from_file_path(&schema_path)
            .map_err(|_| anyhow!("Failed to convert to URL: {}", schema_path.display()))?
            .to_string();

        let worker = WorkerHandle::new(app_handle.clone());

        Ok(Self {
            app_handle,
            persist_path,
            schema_url,
            settings: RwLock::new(settings),
            worker,
            hooks: RwLock::new(Default::default()),
        })
    }

    /// Get an immutable reference to the current settings.
    ///
    /// The returned guard will hold a read lock on the settings until dropped.
    /// It is the caller's responsibility to drop the guard as soon as possible
    /// and never attempt to acquire other locks while holding it to avoid
    /// deadlocks.
    pub fn read(&self) -> RwLockReadGuard<'_, Settings> {
        self.settings.read()
    }

    /// Try to get an immutable reference to the current settings.
    ///
    /// Same as [`Self::read`], but returns `None` if the read lock cannot be
    /// acquired immediately. This is useful in scenarios where blocking is not
    /// acceptable.
    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, Settings>> {
        self.settings.try_read()
    }

    /// Get the path where settings are persisted.
    pub fn persist_path(&self) -> &Path {
        &self.persist_path
    }

    /// Persist the current settings to disk.
    pub fn persist(&self) -> Result<()> {
        let settings = self.settings.read();
        settings.dump(&self.persist_path, &self.schema_url)?;
        Ok(())
    }

    /// Register a hook that will be triggered on theme change.
    ///
    /// The two arguments are respectively the old and new themes.
    pub fn on_theme_change<F>(&self, hook: F)
    where
        F: Fn(&Theme, &Theme) + Send + Sync + 'static,
    {
        let mut hooks = self.hooks.write();
        hooks.on_theme_change.push(Box::new(hook));
    }

    /// Trigger all registered theme change hooks.
    pub(crate) fn trigger_theme_hooks(&self, old: &Theme, new: &Theme) {
        let hooks = self.hooks.read();
        for hook in &hooks.on_theme_change {
            hook(old, new);
        }
    }

    /// Register a hook that will be triggered on canvas interaction mode
    /// change.
    ///
    /// The two arguments are respectively the old and new canvas interaction
    /// modes.
    pub fn on_canvas_imode_change<F>(&self, hook: F)
    where
        F: Fn(&CanvasImode, &CanvasImode) + Send + Sync + 'static,
    {
        let mut hooks = self.hooks.write();
        hooks.on_canvas_imode_change.push(Box::new(hook));
    }

    /// Trigger all registered canvas interaction mode change hooks.
    pub(crate) fn trigger_canvas_imode_hooks(&self, old: &CanvasImode, new: &CanvasImode) {
        let hooks = self.hooks.read();
        for hook in &hooks.on_canvas_imode_change {
            hook(old, new);
        }
    }

    /// Register a hook that will be triggered on shortcut change.
    ///
    /// The first argument is the shortcut action. The second and third
    /// arguments are respectively the old and new shortcuts for that action.
    /// `None` means that no shortcut was/is assigned for that action.
    pub fn on_shortcut_change<F>(&self, hook: F)
    where
        F: Fn(&ShortcutAction, Option<&String>, Option<&String>) + Send + Sync + 'static,
    {
        let mut hooks = self.hooks.write();
        hooks.on_shortcut_change.push(Box::new(hook));
    }

    /// Trigger all registered shortcut change hooks.
    pub(crate) fn trigger_shortcut_hooks(
        &self,
        action: &ShortcutAction,
        old: Option<&String>,
        new: Option<&String>,
    ) {
        let hooks = self.hooks.read();
        for hook in &hooks.on_shortcut_change {
            hook(action, old, new);
        }
    }

    /// Update the settings with a patch generated by a closure.
    ///
    /// The closure is given an immutable reference to the current settings and
    /// must return a [`SettingsPatch`] that describes the changes to be made.
    /// See its documentation for details on how settings patching works. If any
    /// actual changes are made, an [`UpdateEvent`] will be emitted with the
    /// updated settings.
    ///
    /// The registered hooks for changed settings will be triggered by the
    /// worker asynchronously. This is done at best effort, meaning that one
    /// failure will not block other changes from being submitted. Failure to
    /// submit one or more changes to the worker will result in an error being
    /// returned at the end. Failure to trigger the hooks will not result in an
    /// error, and this method does not wait for the hooks to complete.
    pub fn update_with<F>(&self, patch: F) -> Result<()>
    where
        F: FnOnce(&Settings) -> SettingsPatch,
    {
        let mut settings = self.settings.write();
        let patch = patch(&settings);

        let mut tasks = vec![];
        let mut should_emit = false; // Should emit; implies should persist
        let mut should_persist = false; // Should persist only

        if let Some(theme) = patch.theme
            && settings.theme != theme
        {
            let old_theme = std::mem::replace(&mut settings.theme, theme.clone());
            tasks.push(WorkerTask::ThemeChanged {
                old: old_theme,
                new: theme,
            });
            should_emit = true;
        }

        if let Some(canvas_imode) = patch.canvas_imode
            && settings.canvas_imode != canvas_imode
        {
            let old_imode = std::mem::replace(&mut settings.canvas_imode, canvas_imode.clone());
            tasks.push(WorkerTask::CanvasImodeChanged {
                old: old_imode,
                new: canvas_imode,
            });
            should_emit = true;
        }

        if let Some(shortcuts) = patch.shortcuts {
            for (action, shortcut) in shortcuts {
                let old_shortcut = match &shortcut {
                    Some(shortcut) => settings.shortcuts.insert(action.clone(), shortcut.clone()),
                    None => settings.shortcuts.remove(&action),
                };
                if old_shortcut != shortcut {
                    tasks.push(WorkerTask::ShortcutChanged {
                        action,
                        old: old_shortcut,
                        new: shortcut,
                    });
                    should_emit = true;
                }
            }
        }

        if let Some(widgets) = patch.widgets {
            for (id, patch) in widgets {
                match patch {
                    Some(patch) => {
                        let widget = settings.widgets.entry(id).or_insert_with(|| {
                            should_emit = true;
                            Default::default()
                        });
                        should_emit |= widget.apply_patch(patch);
                    },
                    None => should_emit |= settings.widgets.remove(&id).is_some(),
                }
            }
        }

        if let Some(starter_widgets_added) = patch.starter_widgets_added
            && settings.starter_widgets_added != starter_widgets_added
        {
            settings.starter_widgets_added = starter_widgets_added;
            should_persist = true;
        }

        if should_emit {
            UpdateEvent(&settings).emit(&self.app_handle)?;
        }
        if should_emit || should_persist {
            tasks.push(WorkerTask::Persist);
        }

        // TODO: downgrade write lock to read lock when stable on std or when
        // switching to parking_lot
        // We cannot release the lock here; otherwise, a later update may sneak
        // in and tasks will be submitted and processed out of order, causing a
        // race condition
        let mut errors = vec![];
        for task in tasks {
            if let Err(e) = self.worker.process(task) {
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

    /// Update the settings with a patch.
    ///
    /// This is a wrapper of [`Self::update_with`] that takes a fixed patch.
    ///
    /// Tauri command: [`crate::commands::update`].
    pub fn update(&self, patch: SettingsPatch) -> Result<()> {
        self.update_with(|_| patch)
    }
}
