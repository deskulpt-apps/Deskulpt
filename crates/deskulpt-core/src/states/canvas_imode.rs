//! State management for canvas interaction mode.

use std::sync::Mutex;

use anyhow::Result;
use deskulpt_common::event::Event;
use deskulpt_common::window::DeskulptWindow;
use tauri::menu::MenuItem;
use tauri::{App, AppHandle, Emitter, Manager, Runtime, WebviewWindow};
use tracing::{instrument, warn};

use crate::events::ShowToastEvent;

/// Canvas interaction mode.
#[derive(Clone)]
enum CanvasImode {
    /// Sink mode.
    ///
    /// The canvas is click-through. Widgets are not interactable. The desktop
    /// is interactable.
    Sink,
    /// Float mode.
    ///
    /// The canvas is not click-through. Widgets are interactable. The desktop
    /// is not interactable.
    Float,
}

/// The internal of the managed state for canvas interaction mode.
struct CanvasImodeStateInner<R: Runtime> {
    /// The interaction mode of the canvas.
    mode: CanvasImode,
    /// The menu item for toggling the canvas interaction mode.
    menu_item: Option<MenuItem<R>>,
}

impl<R: Runtime> CanvasImodeStateInner<R> {
    /// Toggle the interaction mode.
    ///
    /// This will change the mode and update the menu item text if it exists.
    fn toggle(&mut self, canvas: &WebviewWindow<R>) -> Result<()> {
        // The menu item shows the action that will be performed on click, so it
        // should be the opposite of the mode
        let (new_mode, new_text) = match self.mode {
            CanvasImode::Sink => {
                canvas.set_ignore_cursor_events(false)?;
                (CanvasImode::Float, "Sink")
            },
            CanvasImode::Float => {
                canvas.set_ignore_cursor_events(true)?;
                (CanvasImode::Sink, "Float")
            },
        };

        self.mode = new_mode;
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
            mode: CanvasImode::Sink,
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
    #[instrument(skip(self), err)]
    fn toggle_canvas_imode(&self) -> Result<()>
    where
        Self: Sized,
    {
        let canvas = DeskulptWindow::Canvas.webview_window(self)?;

        let state = self.state::<CanvasImodeState<R>>();
        let mut state = state.0.lock().unwrap();
        state.toggle(&canvas)?;

        let toast_message = match state.mode {
            CanvasImode::Float => "Canvas floated.",
            CanvasImode::Sink => {
                // Toggled from float to sink, so we try to regain focus to
                // avoid flickering on the first click; failure to do so is not
                // critical so we consume the error
                if let Err(e) = canvas.set_focus() {
                    warn!(window = "canvas", error = %e, "Failed to regain focus after toggling");
                }
                "Canvas sunk."
            },
        };

        if let Err(e) =
            ShowToastEvent::Success(toast_message.to_string()).emit_to(self, DeskulptWindow::Canvas)
        {
            warn!(window = "canvas", error = %e, "Failed to emit toast notification");
        }

        Ok(())
    }
}

impl<R: Runtime> CanvasImodeStateExt<R> for App<R> {}
impl<R: Runtime> CanvasImodeStateExt<R> for AppHandle<R> {}
