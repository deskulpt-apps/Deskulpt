//! Deskulpt windows.

pub mod mouse_interaction;
mod script;

use anyhow::Result;
use deskulpt_common::window::DeskulptWindow;
use mouse_interaction::MouseInteractionManager;
use script::{CanvasInitJS, ManagerInitJS};
use tauri::{
    App, AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder, Window, WindowEvent,
};

use crate::settings::Theme;
use crate::states::SettingsStateExt;

/// Extention trait for window-related operations.
pub trait WindowExt<R: Runtime>: Manager<R> + SettingsStateExt<R> {
    /// Create the manager window.
    fn create_manager(&self) -> Result<()>
    where
        Self: Sized,
    {
        let settings = self.get_settings();
        let init_js = ManagerInitJS::generate(&settings)?;

        // https://www.radix-ui.com/colors: "Slate 1" colors
        let background_color = match settings.theme {
            Theme::Light => (252, 252, 253), // #FCFCFD
            Theme::Dark => (17, 17, 19),     // #111113
        };

        WebviewWindowBuilder::new(
            self,
            DeskulptWindow::Manager,
            WebviewUrl::App("packages/deskulpt-manager/index.html".into()),
        )
        .title("Deskulpt Manager")
        .background_color(background_color.into())
        .inner_size(800.0, 500.0)
        .center()
        .resizable(false)
        .maximizable(false)
        .minimizable(false)
        .visible(false)
        .initialization_script(&init_js)
        .build()?;

        Ok(())
    }

    /// Create the canvas window.
    fn create_canvas(&self) -> Result<()>
    where
        Self: Sized,
    {
        let settings = self.get_settings();
        let init_js = CanvasInitJS::generate(&settings)?;
        let canvas = WebviewWindowBuilder::new(
            self,
            DeskulptWindow::Canvas,
            WebviewUrl::App("packages/deskulpt-canvas/index.html".into()),
        )
        .title("Deskulpt Canvas")
        .maximized(true)
        .transparent(true)
        .decorations(false)
        .always_on_bottom(true)
        // TODO: Remove when the following issue is fixed:
        // https://github.com/tauri-apps/tauri/issues/9597
        .visible(false)
        // Unsupported on macOS; see below for activation policy
        .skip_taskbar(true)
        .initialization_script(&init_js)
        .build()?;

        #[cfg(target_os = "macos")]
        {
            use objc2::msg_send;
            use objc2::runtime::{AnyObject, Bool};

            // Disable the window shadow on macOS; there will be shadows left on
            // movement for transparent and undecorated windows that we are using;
            // it seems that disabling shadows does not have significant visual impacts
            unsafe {
                let ns_window = canvas.ns_window()? as *mut AnyObject;
                let () = msg_send![ns_window, setHasShadow:Bool::NO];
            }
        }

        let scale_factor = canvas.scale_factor()?;
        let canvas_cloned = canvas.clone();
        std::thread::spawn(move || {
            // Initialize mouse interaction manager with the window's scale factor
            let mut mouse_manager = MouseInteractionManager::new(scale_factor);

            rdev::listen(move |event| {
                if let rdev::EventType::MouseMove { x, y } = event.event_type {
                    let settings = canvas_cloned.app_handle().get_settings();

                    // Process mouse movement and update cursor interaction state if needed
                    if let Some(should_ignore_cursor) =
                        mouse_manager.process_mouse_move(x, y, &settings.widgets)
                    {
                        if let Err(e) = canvas_cloned.set_ignore_cursor_events(should_ignore_cursor)
                        {
                            eprintln!("Failed to set cursor events state: {e}");
                        } else {
                            println!("ignore_cursor_events: {should_ignore_cursor}");
                        }
                    }
                }
            })
            .unwrap_or_else(|e| {
                eprintln!("Failed to listen for mouse events: {e:?}");
            });
        });

        // TODO: Remove when the following issue is fixed:
        // https://github.com/tauri-apps/tauri/issues/9597
        canvas.show()?;

        // Canvas is by default click-through
        canvas.set_ignore_cursor_events(true)?;

        Ok(())
    }

    /// Open the manager window.
    fn open_manager(&self) -> Result<()> {
        let manager = DeskulptWindow::Manager.webview_window(self)?;
        manager.show()?;
        manager.set_focus()?;
        Ok(())
    }
}

impl<R: Runtime> WindowExt<R> for App<R> {}
impl<R: Runtime> WindowExt<R> for AppHandle<R> {}

/// Window event handler for all Deskulpt windows.
pub fn on_window_event(window: &Window, event: &WindowEvent) {
    if window.label() == "manager" {
        // Prevent the manager window from closing when the close button is
        // clicked, but only hide it instead
        if let WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            if let Err(e) = window.hide() {
                eprintln!("Failed to hide the manager window: {e}");
            }
        }
    }
}
