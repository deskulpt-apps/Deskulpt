//! Keyboard shortcut management.

use anyhow::Result;
use deskulpt_settings::{SettingsExt, ShortcutAction};
use tauri::{App, AppHandle, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcut, GlobalShortcutExt, ShortcutState};

use crate::states::CanvasImodeStateExt;
use crate::window::WindowExt;

/// Re-register a shortcut.
///
/// The old shortcut will be unregistered and the new shortcut will be
/// registered, with the listener determined by the shortcut action.
fn reregister_shortcut<R: Runtime>(
    gs: &GlobalShortcut<R>,
    action: &ShortcutAction,
    old: Option<&String>,
    new: Option<&String>,
) -> Result<()> {
    if let Some(shortcut) = old {
        gs.unregister(shortcut.as_str())?;
    }

    let handler: fn(&AppHandle<R>) = match action {
        ShortcutAction::ToggleCanvasImode => |app_handle| {
            if let Err(e) = app_handle.toggle_canvas_imode() {
                tracing::error!("Failed to toggle canvas interaction mode: {e}");
            }
        },
        ShortcutAction::OpenManager => |app_handle| {
            if let Err(e) = app_handle.open_manager() {
                tracing::error!("Failed to open the manager window: {e}");
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
pub trait ShortcutsExt<R: Runtime>: Manager<R> + SettingsExt<R> + GlobalShortcutExt<R> {
    /// Initialize keyboard shortcuts management.
    ///
    /// This immediately registers shortcuts based on the settings. Failure to
    /// register the shortcuts are properly logged but not fatal. It also
    /// re-registers shortcuts when shortcuts in the settings change.
    fn init_shortcuts(&self) {
        {
            let gs = self.global_shortcut();
            let settings = self.settings().read();
            for (action, shortcut) in &settings.shortcuts {
                if let Err(e) = reregister_shortcut(gs, action, None, Some(shortcut)) {
                    tracing::error!(
                        shortcut_action = ?action,
                        shortcut_binding = %shortcut,
                        error = ?e,
                        "Failed to register shortcut",
                    );
                }
            }
        }

        let app_handle = self.app_handle().clone();
        self.settings().on_shortcut_change(move |action, old, new| {
            let gs = app_handle.global_shortcut();
            if let Err(e) = reregister_shortcut(gs, action, old, new) {
                tracing::error!(
                    "Failed to re-register shortcut from {old:?} to {new:?} for {action:?}: {e:?}"
                );
            }
        });
    }
}

impl<R: Runtime> ShortcutsExt<R> for App<R> {}
impl<R: Runtime> ShortcutsExt<R> for AppHandle<R> {}
