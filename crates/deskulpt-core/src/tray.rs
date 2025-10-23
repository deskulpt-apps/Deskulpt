//! Deskulpt system tray.

use anyhow::Result;
use tauri::image::Image;
use tauri::menu::{MenuBuilder, MenuEvent, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{App, AppHandle, Runtime};

use crate::path::PathExt;
use crate::settings::{CanvasImode, SettingsPatch};
use crate::states::{CanvasImodeStateExt, SettingsStateExt};
use crate::window::WindowExt;

/// Extention trait for system tray-related operations.
pub trait TrayExt<R: Runtime>: CanvasImodeStateExt<R> + SettingsStateExt<R> {
    /// Create the system tray.
    fn create_tray(&self, icon: Image) -> Result<()>
    where
        Self: Sized,
    {
        // Store the menu item for toggling canvas interaction mode
        let text = match self.get_settings().canvas_imode {
            crate::settings::CanvasImode::Sink => "Float",
            crate::settings::CanvasImode::Float => "Sink",
        };
        let menu_item_toggle = MenuItemBuilder::with_id("tray-toggle", text).build(self)?;
        self.set_canvas_imode_menu_item(&menu_item_toggle);

        // Build the system tray menu
        let tray_menu = MenuBuilder::new(self)
            .items(&[
                &menu_item_toggle,
                &MenuItemBuilder::with_id("tray-manage", "Manage").build(self)?,
                &MenuItemBuilder::with_id("tray-exit", "Exit").build(self)?,
            ])
            .build()?;

        // Build the system tray icon
        TrayIconBuilder::with_id("tray")
            .icon(icon)
            .icon_as_template(true)
            .show_menu_on_left_click(false)
            .tooltip("Deskulpt")
            .menu(&tray_menu)
            .on_menu_event(on_menu_event)
            .on_tray_icon_event(on_tray_icon_event)
            .build(self)?;

        Ok(())
    }
}

impl<R: Runtime> TrayExt<R> for App<R> {}
impl<R: Runtime> TrayExt<R> for AppHandle<R> {}

/// Handler for system tray menu events.
///
/// This handler will receive any menu event but only act on events related to
/// the system tray.
fn on_menu_event<R: Runtime>(app_handle: &AppHandle<R>, event: MenuEvent) {
    match event.id().as_ref() {
        "tray-toggle" => {
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
        "tray-manage" => {
            if let Err(e) = app_handle.open_manager() {
                eprintln!("Failed to open manager window: {e}");
            }
        },
        "tray-exit" => {
            if let Err(e) = app_handle
                .persist_dir()
                .and_then(|dir| app_handle.get_settings().dump(dir))
            {
                eprintln!("Failed to dump settings before exit: {e}");
                app_handle.exit(1);
                return;
            }
            app_handle.exit(0);
        },
        _ => {},
    }
}

/// Handler for system tray icon events.
fn on_tray_icon_event<R: Runtime>(tray: &TrayIcon<R>, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button,
        button_state,
        ..
    } = event
    {
        if button == MouseButton::Left && button_state == MouseButtonState::Down {
            // Toggle canvas interaction mode on left-click
            let app_handle = tray.app_handle().clone();
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
        }
    }
}
