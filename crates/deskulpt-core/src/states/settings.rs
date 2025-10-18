//! State management for the settings.

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use anyhow::{bail, Result};
use tauri::{App, AppHandle, Manager, Runtime};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

use crate::path::PathExt;
use crate::settings::{Settings, SettingsPatch};

/// Managed state for the settings.
struct SettingsState(RwLock<Settings>);

/// Extension trait for operations on the settings state.
pub trait SettingsStateExt<R: Runtime>: Manager<R> + PathExt<R> + GlobalShortcutExt<R> {
    /// Initialize state management for the settings.
    ///
    /// This will load the settings from the persistence directory and
    /// initialize the shortcuts. If any step fails, it will fall back to a
    /// state that preserves as much persisted data as possible.
    fn manage_settings(&self) {
        let mut settings = self
            .persist_dir()
            .and_then(Settings::load)
            .unwrap_or_else(|e| {
                eprintln!("Failed to load settings: {e}");
                Settings::default()
            });
        settings.init_shortcuts(self.global_shortcut());
        self.manage(SettingsState(RwLock::new(settings)));
    }

    /// Get an immutable reference to the settings.
    ///
    /// The returned reference is behind a lock guard, which should be dropped
    /// as soon as possible to minimize critical section.
    fn get_settings(&self) -> RwLockReadGuard<'_, Settings> {
        let state = self.state::<SettingsState>().inner();
        state.0.read().unwrap()
    }

    /// Get a mutable reference to the settings.
    ///
    /// The returned reference is behind a lock guard, which should be dropped
    /// as soon as possible to minimize critical section.
    fn get_settings_mut(&self) -> RwLockWriteGuard<'_, Settings> {
        let state = self.state::<SettingsState>().inner();
        state.0.write().unwrap()
    }

    /// Apply a patch to the settings.
    ///
    /// See [`SettingsPatch`] for more information of how the patch is applied.
    /// The patch application is best-effort: any part of the patch that fails
    /// to be applied will be skipped, and the rest will be applied as normal.
    /// Errors will be accumulated and returned as a single error at the end if
    /// any occurred.
    fn apply_settings_patch(&self, patch: SettingsPatch) -> Result<()> {
        let mut errors = vec![];
        let mut settings = self.get_settings_mut();

        if let Some(theme) = patch.theme {
            settings.theme = theme;
        }

        if let Some(shortcuts) = patch.shortcuts {
            let gs = self.global_shortcut();
            for (key, shortcut) in shortcuts {
                if let Err(e) = settings.update_shortcut(gs, &key, shortcut) {
                    errors.push(e.context(format!("Failed to update /shortcuts/{key:?}")));
                }
            }
        }

        if let Some(widgets) = patch.widgets {
            for (id, patch) in widgets {
                if patch.is_none() {
                    settings.widgets.remove(&id);
                    continue;
                }
                let patch = patch.unwrap();
                let widget = settings.widgets.entry(id).or_default();

                if let Some(x) = patch.x {
                    widget.x = x;
                }
                if let Some(y) = patch.y {
                    widget.y = y;
                }
                if let Some(width) = patch.width {
                    widget.width = width;
                }
                if let Some(height) = patch.height {
                    widget.height = height;
                }
                if let Some(opacity) = patch.opacity {
                    widget.opacity = opacity;
                }
            }
        }

        if errors.is_empty() {
            return Ok(());
        }

        let message = errors
            .into_iter()
            .map(|e| format!("{e:?}"))
            .collect::<Vec<_>>()
            .join("\n\n");
        bail!("One or more errors occurred while applying the settings patch:\n\n{message}");
    }
}

impl<R: Runtime> SettingsStateExt<R> for App<R> {}
impl<R: Runtime> SettingsStateExt<R> for AppHandle<R> {}
