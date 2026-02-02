//! Deskulpt windows.

mod script;

use anyhow::Result;
use deskulpt_common::window::DeskulptWindow;
use deskulpt_settings::SettingsExt;
use deskulpt_settings::model::{CanvasImode, Theme};
use script::{CanvasInitJS, PortalInitJS};
use tauri::{App, AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder, WindowEvent};

use crate::states::CanvasImodeStateExt;

/// Extention trait for window-related operations.
pub trait WindowExt<R: Runtime>: Manager<R> + SettingsExt<R> {
    /// Open Deskulpt portal.
    ///
    /// If the portal already exists, it will be focused. Otherwise it will be
    /// created first.
    fn open_portal(&self) -> Result<()>
    where
        Self: Sized,
    {
        if let Ok(portal) = DeskulptWindow::Portal.webview_window(self) {
            portal.set_focus()?;
            return Ok(());
        }

        let settings = self.settings().read();
        let init_js = PortalInitJS::generate(&settings)?;

        // https://www.radix-ui.com/colors: "Slate 1" colors
        let background_color = match settings.theme {
            Theme::Light => (252, 252, 253), // #FCFCFD
            Theme::Dark => (17, 17, 19),     // #111113
        };

        let portal = WebviewWindowBuilder::new(
            self,
            DeskulptWindow::Portal,
            WebviewUrl::App("packages/deskulpt-portal/index.html".into()),
        )
        .title("Deskulpt Portal")
        .background_color(background_color.into())
        .inner_size(800.0, 500.0)
        .center()
        .resizable(false)
        .maximizable(false)
        .minimizable(false)
        .initialization_script(&init_js)
        .build()?;

        portal.set_focus()?;

        Ok(())
    }

    /// Create Deskulpt canvas.
    fn create_canvas(&self) -> Result<()>
    where
        Self: Sized,
    {
        let settings = self.settings().read();
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
        .shadow(false)
        .build()?;

        // TODO: Remove when the following issue is fixed:
        // https://github.com/tauri-apps/tauri/issues/9597
        canvas.show()?;

        let app_handle = self.app_handle().clone();
        canvas.on_window_event(move |event| match event {
            WindowEvent::Moved(position) => {
                app_handle.set_canvas_position(position);
            },
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                app_handle.set_canvas_scale_factor(*scale_factor);
            },
            _ => {},
        });

        if settings.canvas_imode == CanvasImode::Sink {
            canvas.set_ignore_cursor_events(true)?;
        }

        Ok(())
    }
}

impl<R: Runtime> WindowExt<R> for App<R> {}
impl<R: Runtime> WindowExt<R> for AppHandle<R> {}
