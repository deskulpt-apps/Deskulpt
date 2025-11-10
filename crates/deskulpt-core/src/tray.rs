//! Deskulpt system tray.

use anyhow::Result;
use deskulpt_settings::SettingsExt;
use tauri::image::Image;
use tauri::menu::{MenuBuilder, MenuEvent, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{App, AppHandle, Runtime};
use tracing::instrument;

use crate::states::CanvasImodeStateExt;
use crate::window::WindowExt;

/// Extention trait for system tray-related operations.
pub trait TrayExt<R: Runtime>: CanvasImodeStateExt<R> {
    /// Create the system tray.
    #[instrument(skip(self, icon), err)]
    fn create_tray(&self, icon: Image) -> Result<()>
    where
        Self: Sized,
    {
        // Store the menu item for toggling canvas interaction mode
        let menu_item_toggle = MenuItemBuilder::with_id("tray-toggle", "Float").build(self)?;
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
#[instrument(skip(app_handle, event))]
fn on_menu_event<R: Runtime>(app_handle: &AppHandle<R>, event: MenuEvent) {
    match event.id().as_ref() {
        "tray-toggle" => {
            if let Err(e) = app_handle.toggle_canvas_imode() {
                tracing::error!(
                    error = %e,
                    "Failed to toggle canvas interaction mode from tray menu",
                );
            };
        },
        "tray-manage" => {
            if let Err(e) = app_handle.open_manager() {
                tracing::error!(
                    error = %e,
                    "Failed to open manager window from tray menu",
                );
            };
        },
        "tray-exit" => {
            let persist_result = app_handle
                .persist_dir()
                .and_then(|dir| app_handle.get_settings().dump(dir));
            if let Err(e) = persist_result {
                tracing::error!(
                    error = %e,
                    "Failed to persist settings before exiting from tray",
                );
                app_handle.exit(1);
                return;
            }
            app_handle.exit(0);
        },
        _ => {},
    }
}

/// Handler for system tray icon events.
#[instrument(skip(tray, event))]
fn on_tray_icon_event<R: Runtime>(tray: &TrayIcon<R>, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button,
        button_state,
        ..
    } = event
        && button == MouseButton::Left
        && button_state == MouseButtonState::Down
    {
        // Toggle canvas interaction mode on left-click
        if let Err(e) = tray.app_handle().toggle_canvas_imode() {
            tracing::error!(
                error = %e,
                "Failed to toggle canvas interaction mode from tray icon",
            );
        }
    }
}
