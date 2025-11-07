//! Keyboard shortcut management.

use anyhow::Result;
use deskulpt_settings::{SettingsExt, ShortcutAction};
use tauri::{App, AppHandle, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcut, GlobalShortcutExt, ShortcutState};
use tracing::info_span;

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

    let handler: fn(&AppHandle<R>) = match key {
        ShortcutKey::ToggleCanvasImode => |app_handle| {
            let span = info_span!(
                "shortcut.invoke",
                shortcut_key = "toggleCanvasImode",
                operation = "toggle_canvas_imode",
                trigger = "shortcut",
                status = tracing::field::Empty,
            );
            let result = span.in_scope(|| app_handle.toggle_canvas_imode());
            if let Err(e) = result {
                tracing::error!(
                    shortcut_key = "toggleCanvasImode",
                    operation = "toggle_canvas_imode",
                    trigger = "shortcut",
                    status = "error",
                    error_kind = %e,
                    "Failed to toggle canvas interaction mode from shortcut",
                );
            }
        },
        ShortcutKey::OpenManager => |app_handle| {
            let span = info_span!(
                "shortcut.invoke",
                shortcut_key = "openManager",
                operation = "open_manager_window",
                trigger = "shortcut",
                status = tracing::field::Empty,
            );
            let result = span.in_scope(|| app_handle.open_manager());
            if let Err(e) = result {
                tracing::error!(
                    shortcut_key = "openManager",
                    operation = "open_manager_window",
                    trigger = "shortcut",
                    status = "error",
                    error_kind = %e,
                    "Failed to open the manager window from shortcut",
                );
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
            for (key, shortcut) in &settings.shortcuts {
                if let Err(e) = reregister_shortcut(gs, key, None, Some(shortcut)) {
                    tracing::error!(
                        shortcut_key = ?key,
                        shortcut_binding = %shortcut,
                        operation = "register_shortcut",
                        status = "error",
                        error_kind = ?e,
                        "Failed to register shortcut",
                    );
                }
            }
        }

        let app_handle = self.app_handle().clone();
        self.settings().on_shortcut_change(move |action, old, new| {
            let gs = app_handle.global_shortcut();
            if let Err(e) = reregister_shortcut(gs, key, old, new) {
                tracing::error!(
                    shortcut_key = ?key,
                    old_binding = ?old,
                    new_binding = ?new,
                    operation = "register_shortcut",
                    status = "error",
                    error_kind = ?e,
                    "Failed to re-register shortcut",
                );
            }
        });
    }
}

impl<R: Runtime> ShortcutsExt<R> for App<R> {}
impl<R: Runtime> ShortcutsExt<R> for AppHandle<R> {}
