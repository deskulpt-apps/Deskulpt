//! State management for canvas interaction mode.

use std::sync::Mutex;

use anyhow::Result;
use deskulpt_common::event::Event;
use deskulpt_common::window::DeskulptWindow;
use tauri::menu::MenuItem;
use tauri::{App, AppHandle, Emitter, Manager, Runtime, WebviewWindow};

use crate::events::ShowToastEvent;
use crate::settings::CanvasImode;

/// The internal of the managed state for canvas interaction mode.
struct CanvasImodeStateInner<R: Runtime> {
    /// The menu item for toggling the canvas interaction mode.
    menu_item: Option<MenuItem<R>>,
}

impl<R: Runtime> CanvasImodeStateInner<R> {
    /// Toggle the interaction mode.
    ///
    /// This will change the mode and update the menu item text if it exists.
    fn toggle(&mut self, canvas: &WebviewWindow<R>, mode: &CanvasImode) -> Result<()> {
        // The menu item shows the action that will be performed on click, so it
        // should be the opposite of the mode
        let new_text = match mode {
            CanvasImode::Sink => {
                canvas.set_ignore_cursor_events(true)?;
                "Float"
            },
            CanvasImode::Float => {
                canvas.set_ignore_cursor_events(false)?;
                "Sink"
            },
        };

        if let Some(menu_item) = &self.menu_item {
            menu_item.set_text(new_text)?;
        }
        Ok(())
    }
}

/// Managed state for canvas interaction mode.
struct CanvasImodeState<R: Runtime>(Mutex<CanvasImodeStateInner<R>>);

/// Extension trait for operations on canvas interaction mode.
pub trait CanvasImodeStateExt<R: Runtime>: Manager<R> + Emitter<R> {
    /// Initialize state management for canvas interaction mode.
    ///
    /// The canvas is in sink mode by default.
    fn manage_canvas_imode(&self) {
        let inner = CanvasImodeStateInner {
            menu_item: None::<MenuItem<R>>,
        };
        self.manage(CanvasImodeState(Mutex::new(inner)));
    }

    /// Set the menu item for toggling canvas interaction mode.
    fn set_canvas_imode_menu_item(&self, menu_item: &MenuItem<R>) {
        let state = self.state::<CanvasImodeState<R>>();
        let mut state = state.0.lock().unwrap();

        // Cloning works because menu items are behind shared references
        state.menu_item = Some(menu_item.clone());
    }

    /// Toggle the interaction mode of the canvas window.
    ///
    /// This will show a toast message on the canvas window indicating the new
    /// interaction mode.
    fn toggle_canvas_imode(&self, mode: &CanvasImode) -> Result<()>
    where
        Self: Sized,
    {
        let canvas = DeskulptWindow::Canvas.webview_window(self)?;

        let state = self.state::<CanvasImodeState<R>>();
        let mut state = state.0.lock().unwrap();

        state.toggle(&canvas, mode)?;

        let toast_message = match mode {
            CanvasImode::Float => "Canvas floated.",
            CanvasImode::Sink => {
                // Toggled from float to sink, so we try to regain focus to
                // avoid flickering on the first click; failure to do so is not
                // critical so we consume the error
                if let Err(e) = canvas.set_focus() {
                    eprintln!("Failed to gain focus on canvas: {}", e);
                }
                "Canvas sunk."
            },
        };

        if let Err(e) =
            ShowToastEvent::Success(toast_message.to_string()).emit_to(self, DeskulptWindow::Canvas)
        {
            eprintln!("Failed to emit ShowToastEvent to canvas: {}", e);
        }

        Ok(())
    }
}

impl<R: Runtime> CanvasImodeStateExt<R> for App<R> {}
impl<R: Runtime> CanvasImodeStateExt<R> for AppHandle<R> {}
