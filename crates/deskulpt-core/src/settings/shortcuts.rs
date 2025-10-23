//! Keyboard shortcut management.

use anyhow::Result;
use tauri::{AppHandle, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcut, ShortcutState};

use super::{Settings, ShortcutKey};
use crate::settings::{CanvasImode, SettingsPatch};
use crate::states::SettingsStateExt;
use crate::window::WindowExt;

/// Helper function for re-registering a keyboard shortcut.
///
/// If the old and new shortcuts are the same, this is a no-op. Otherwise, the
/// old shortcut will be unregistered and the new shortcut will be registered
/// with the listener determined by the shortcut key.
fn reregister_shortcut<R: Runtime>(
    gs: &GlobalShortcut<R>,
    key: &ShortcutKey,
    old: Option<&str>,
    new: Option<&str>,
) -> Result<()> {
    if old == new {
        return Ok(());
    }

    if let Some(shortcut) = old {
        gs.unregister(shortcut)?;
    }

    let handler: fn(&AppHandle<R>) = match key {
        ShortcutKey::ToggleCanvasImode => |app_handle| {
            let new_mode = match app_handle.get_settings().canvas_imode {
                CanvasImode::Sink => CanvasImode::Float,
                CanvasImode::Float => CanvasImode::Sink,
            };
            app_handle
                .update_settings(SettingsPatch {
                    canvas_imode: Some(new_mode),
                    ..Default::default()
                })
                .unwrap_or_else(|e| {
                    eprintln!("Failed to toggle canvas interaction mode: {e}");
                });
        },
        ShortcutKey::OpenManager => |app_handle| {
            if let Err(e) = app_handle.open_manager() {
                eprintln!("Failed to open the manager window: {e}");
            }
        },
    };

    if let Some(shortcut) = new {
        gs.on_shortcut(shortcut, move |app_handle, _, event| {
            if event.state == ShortcutState::Pressed {
                handler(app_handle);
            }
        })?;
    }

    Ok(())
}

impl Settings {
    /// Initialize keyboard shortcuts.
    ///
    /// This method attempts to register all shortcuts defined in the settings.
    /// If any registration fails, instead of bailing out, it will remove that
    /// shortcut from the settings and silently continue.
    pub fn init_shortcuts<R: Runtime>(&mut self, gs: &GlobalShortcut<R>) {
        for (key, current) in self.shortcuts.clone() {
            if let Err(e) = reregister_shortcut(gs, &key, None, Some(&current)) {
                eprintln!("Failed to register shortcut for {key:?}: {e}");
                self.shortcuts.remove(&key);
            }
        }
    }

    /// Update a keyboard shortcut.
    ///
    /// If the new shortcut is not `None`, the existing shortcut will be
    /// replaced. Otherwise, the existing shortcut will be removed. The
    /// involved shortcuts will be automatically unregistered or registered.
    pub fn update_shortcut<R: Runtime>(
        &mut self,
        gs: &GlobalShortcut<R>,
        key: &ShortcutKey,
        new: Option<String>,
    ) -> Result<()> {
        reregister_shortcut(
            gs,
            key,
            self.shortcuts.get(key).map(|s| s.as_str()),
            new.as_deref(),
        )?;

        if let Some(shortcut) = new {
            self.shortcuts.insert(key.clone(), shortcut);
        } else {
            self.shortcuts.remove(key);
        }

        Ok(())
    }
}
