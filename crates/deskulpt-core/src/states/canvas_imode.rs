//! State management for canvas interaction mode.

use anyhow::Result;
use deskulpt_common::event::Event;
use deskulpt_common::window::DeskulptWindow;
use deskulpt_settings::{CanvasImode, SettingsExt, SettingsPatch};
use tauri::menu::MenuItem;
use tauri::{App, AppHandle, Emitter, Manager, Runtime, WebviewWindow};

use crate::events::ShowToastEvent;

/// Managed state for canvas interaction mode.
struct CanvasImodeState<R: Runtime>(MenuItem<R>);

/// Extension trait for operations on canvas interaction mode.
pub trait CanvasImodeStateExt<R: Runtime>: Manager<R> + Emitter<R> + SettingsExt<R> {
    /// Initialize state management for canvas interaction mode.
    ///
    /// This will also listen for changes to the canvas interaction mode and
    /// update the canvas window and menu item accordingly.
    fn manage_canvas_imode(&self, menu_item: MenuItem<R>) -> Result<()> {
        self.manage(CanvasImodeState(menu_item));

        let app_handle = self.app_handle().clone();
        let canvas = DeskulptWindow::Canvas.webview_window(&app_handle)?;
        self.settings().on_canvas_imode_change(move |_, new| {
            if let Err(e) = on_new_canvas_imode(&app_handle, &canvas, new) {
                tracing::error!("Failed to update canvas interaction mode: {}", e);
            }
        });

        Ok(())
    }

    /// Toggle the interaction mode of the canvas window.
    fn toggle_canvas_imode(&self) -> Result<()> {
        self.settings().update_with(|settings| SettingsPatch {
            canvas_imode: Some(match settings.canvas_imode {
                CanvasImode::Float => CanvasImode::Sink,
                CanvasImode::Sink => CanvasImode::Float,
            }),
            ..Default::default()
        })?;
        Ok(())
    }
}

impl<R: Runtime> CanvasImodeStateExt<R> for App<R> {}
impl<R: Runtime> CanvasImodeStateExt<R> for AppHandle<R> {}

/// Handler for canvas interaction mode changes.
///
/// This updates the canvas window's click-through state and the menu item
/// text. It also emits a toast notification to the canvas window, but failure
/// to do so is non-fatal and will not result in an error.
fn on_new_canvas_imode<R: Runtime>(
    app_handle: &AppHandle<R>,
    canvas: &WebviewWindow<R>,
    mode: &CanvasImode,
) -> Result<()> {
    let (should_ignore, menu_text) = match mode {
        CanvasImode::Sink => (true, "Float"),
        CanvasImode::Float => (false, "Sink"),
    };
    canvas.set_ignore_cursor_events(should_ignore)?;

    let state = app_handle.state::<CanvasImodeState<R>>();
    state.0.set_text(menu_text)?;

    if let Err(e) = ShowToastEvent::Success(format!("Canvas interaction mode: {mode:?}"))
        .emit_to(app_handle, DeskulptWindow::Canvas)
    {
        tracing::error!("Failed to emit ShowToastEvent to canvas: {}", e);
    }

    Ok(())
}
