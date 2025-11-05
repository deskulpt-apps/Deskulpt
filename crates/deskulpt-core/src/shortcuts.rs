//! Keyboard shortcut management.

use anyhow::Result;
use tauri::{App, AppHandle, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcut, GlobalShortcutExt, ShortcutState};

use crate::settings::ShortcutKey;
use crate::states::{CanvasImodeStateExt, SettingsStateExt};
use crate::window::WindowExt;

/// Re-register a shortcut.
///
/// The old shortcut will be unregistered and the new shortcut will be
/// registered, with the listener determined by the shortcut key.
fn reregister_shortcut<R: Runtime>(
    gs: &GlobalShortcut<R>,
    key: &ShortcutKey,
    old: Option<&String>,
    new: Option<&String>,
) -> Result<()> {
    if let Some(shortcut) = old {
        gs.unregister(shortcut.as_str())?;
    }

    let handler: fn(&AppHandle<R>) = match key {
        ShortcutKey::ToggleCanvasImode => |app_handle| {
            if let Err(e) = app_handle.toggle_canvas_imode() {
                eprintln!("Failed to toggle canvas interaction mode: {e}");
            }
        },
        ShortcutKey::OpenManager => |app_handle| {
            if let Err(e) = app_handle.open_manager() {
                eprintln!("Failed to open the manager window: {e}");
            }
        },
    };

    if let Some(shortcut) = new {
        gs.on_shortcut(shortcut.as_str(), move |app_handle, _, event| {
            if event.state == ShortcutState::Pressed {
                handler(app_handle);
            }
        })?;
    }

    Ok(())
}

/// Extension trait for keyboard shortcut operations.
pub trait ShortcutsExt<R: Runtime>: SettingsStateExt<R> + GlobalShortcutExt<R> {
    /// Initialize keyboard shortcuts management.
    ///
    /// This immediately registers shortcuts based on the settings. Failure to
    /// register the shortcuts are properly logged but not fatal. It also
    /// re-registers shortcuts when shortcuts in the settings change.
    fn init_shortcuts(&self) {
        {
            let settings = self.get_settings();
            let gs = self.global_shortcut();
            for (key, shortcut) in &settings.shortcuts {
                if let Err(e) = reregister_shortcut(gs, key, None, Some(shortcut)) {
                    eprintln!("Failed to register shortcut {shortcut:?} for {key:?}: {e:?}");
                }
            }
        }

        let app_handle = self.app_handle().clone();
        self.on_shortcut_change(move |key, old, new| {
            let gs = app_handle.global_shortcut();
            if let Err(e) = reregister_shortcut(gs, key, old, new) {
                eprintln!(
                    "Failed to re-register shortcut from {old:?} to {new:?} for {key:?}: {e:?}"
                );
            }
        });
    }
}

impl<R: Runtime> ShortcutsExt<R> for App<R> {}
impl<R: Runtime> ShortcutsExt<R> for AppHandle<R> {}
