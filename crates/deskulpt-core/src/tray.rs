//! Deskulpt system tray.

use anyhow::Result;
use deskulpt_settings::SettingsExt;
use tauri::menu::{MenuBuilder, MenuEvent, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{App, AppHandle, Manager, Runtime};
use tracing::error;

use crate::window::WindowExt;

/// Extention trait for system tray-related operations.
pub trait TrayExt<R: Runtime>: Manager<R> {
    /// Create the system tray.
    fn create_tray(&self) -> Result<()>
    where
        Self: Sized,
    {
        let tray_menu = MenuBuilder::new(self)
            .items(&[
                &MenuItemBuilder::with_id("tray-open-portal", "Portal").build(self)?,
                &MenuItemBuilder::with_id("tray-exit", "Exit").build(self)?,
            ])
            .build()?;

        // Build the system tray icon
        let icon = self
            .app_handle()
            .default_window_icon()
            .expect("No default window icon");
        TrayIconBuilder::with_id("tray")
            .icon(icon.clone())
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
        "tray-open-portal" => {
            if let Err(e) = app_handle.open_portal() {
                error!("Failed to open Deskulpt portal: {e}");
            }
        },
        "tray-exit" => {
            if let Err(e) = app_handle.settings().persist() {
                error!("Failed to persist settings before exit: {e}");
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
        && button == MouseButton::Left
        && button_state == MouseButtonState::Down
        && let Err(e) = tray.app_handle().open_portal()
    {
        error!("Failed to open Deskulpt portal: {e}");
    }
}
