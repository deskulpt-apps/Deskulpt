//! State management for canvas interaction mode.

use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::Result;
use deskulpt_common::event::Event;
use deskulpt_common::window::DeskulptWindow;
use deskulpt_settings::{CanvasImode, SettingsExt, SettingsPatch};
use tauri::{App, AppHandle, Manager, Runtime, WebviewWindow};

use crate::events::ShowToastEvent;

struct CanvasInfo {
    x: f64,
    y: f64,
    scale_factor: f64,
}

/// Managed state for canvas interaction mode.
struct CanvasImodeState {
    lock: RwLock<()>,
    info: RwLock<CanvasInfo>,
}

/// Whether the global mousemove listener is enabled.
static LISTENING_MOUSEMOVE: AtomicBool = AtomicBool::new(false);

/// Extension trait for operations on canvas interaction mode.
pub trait CanvasImodeStateExt<R: Runtime>: Manager<R> + SettingsExt<R> {
    /// Initialize state management for canvas interaction mode.
    ///
    /// This will also hook into settings changes and global mousemove events
    /// and update the canvas window's interaction mode accordingly.
    fn manage_canvas_imode(&self) -> Result<()> {
        let canvas = DeskulptWindow::Canvas.webview_window(self)?;
        let canvas_position = canvas.inner_position()?;
        let canvas_scale_factor = canvas.scale_factor()?;
        let canvas_info = CanvasInfo {
            x: canvas_position.x as f64,
            y: canvas_position.y as f64,
            scale_factor: canvas_scale_factor,
        };
        self.manage(CanvasImodeState {
            lock: RwLock::new(()),
            info: RwLock::new(canvas_info),
        });

        let canvas_cloned = canvas.clone();
        std::thread::spawn(move || {
            if let Err(e) = listen_to_mousemove(canvas_cloned) {
                eprintln!("Failed to listen to global mousemove events: {}", e);
            }
        });

        if self.settings().read().canvas_imode == CanvasImode::Auto {
            LISTENING_MOUSEMOVE.store(true, Ordering::Release);
        }

        self.settings().on_canvas_imode_change(move |_, new| {
            if let Err(e) = on_new_canvas_imode(&canvas, new) {
                eprintln!("Failed to update canvas interaction mode: {}", e);
            }
        });

        Ok(())
    }

    fn set_canvas_info(&self, x: Option<f64>, y: Option<f64>, scale_factor: Option<f64>) {
        let state = self.state::<CanvasImodeState>();
        let mut info = state.info.write().unwrap();

        if let Some(x) = x {
            info.x = x;
        }
        if let Some(y) = y {
            info.y = y;
        }
        if let Some(scale_factor) = scale_factor {
            info.scale_factor = scale_factor;
        }
    }

    /// Toggle the interaction mode of the canvas window.
    ///
    /// If the current mode is float or sink, it switches to the other mode. If
    /// the current mode is auto, it is no-op since auto mode is not toggleable.
    fn toggle_canvas_imode(&self) -> Result<()> {
        self.settings().update_with(|settings| SettingsPatch {
            canvas_imode: match settings.canvas_imode {
                CanvasImode::Auto => None,
                CanvasImode::Float => Some(CanvasImode::Sink),
                CanvasImode::Sink => Some(CanvasImode::Float),
            },
            ..Default::default()
        })?;
        Ok(())
    }
}

impl<R: Runtime> CanvasImodeStateExt<R> for App<R> {}
impl<R: Runtime> CanvasImodeStateExt<R> for AppHandle<R> {}

/// Handler for canvas interaction mode changes.
///
/// This updates the canvas window's click-through state and the mousemove event
/// listener's behavior according to the given mode. It also emits a toast
/// notification to the canvas window, but failure to do so is non-fatal and
/// will not result in an error.
fn on_new_canvas_imode<R: Runtime>(canvas: &WebviewWindow<R>, mode: &CanvasImode) -> Result<()> {
    match mode {
        CanvasImode::Auto => {
            LISTENING_MOUSEMOVE.store(true, Ordering::Release);
        },
        CanvasImode::Sink | CanvasImode::Float => {
            // Set the flag with write lock acquired to avoid racing with the
            // mousemove hook on setting `ignore_cursor_events`
            let state = canvas.state::<CanvasImodeState>();
            let _guard = state.lock.write().unwrap();
            LISTENING_MOUSEMOVE.store(false, Ordering::Release);
            canvas.set_ignore_cursor_events(*mode == CanvasImode::Sink)?;
        },
    }

    if let Err(e) = ShowToastEvent::Success(format!("Canvas interaction mode: {mode:?}"))
        .emit_to(canvas, DeskulptWindow::Canvas)
    {
        eprintln!("Failed to emit ShowToastEvent to canvas: {}", e);
    }

    Ok(())
}

/// Global mousemove event listener.
///
/// If the cheap check on [`LISTENING_MOUSEMOVE`] gives false, the hook will
/// short-circuit immediately, effectively disabling the listener. Otherwise,
/// it will check whether the mouse is over any widget in the canvas window. If
/// so, the canvas will accept cursor events; otherwise, it will ignore them.
fn listen_to_mousemove<R: Runtime>(canvas: WebviewWindow<R>) -> Result<()> {
    let mut is_cursor_ignored = true;

    global_mousemove::listen(move |event| {
        if !LISTENING_MOUSEMOVE.load(Ordering::Acquire) {
            return;
        }

        let (canvas_x, canvas_y, scale_factor) =
            match canvas.state::<CanvasImodeState>().info.try_read() {
                Ok(info) => (info.x, info.y, info.scale_factor),
                Err(_) => return, // Avoid blocking
            };

        #[cfg(target_os = "macos")]
        let scale_factor = 1.0;

        let global_mousemove::MouseMoveEvent { x, y } = event;
        let scaled_x = (x - canvas_x) / scale_factor;
        let scaled_y = (y - canvas_y) / scale_factor;

        let settings = match canvas.settings().try_read() {
            Some(settings) => settings,
            None => return, // Avoid blocking
        };
        let mouse_over_widget = settings.widgets.values().any(|widget| {
            scaled_x >= widget.x as f64
                && scaled_x < widget.x as f64 + widget.width as f64
                && scaled_y >= widget.y as f64
                && scaled_y < widget.y as f64 + widget.height as f64
        });

        // Avoid redundant calls by checking if the state has really changed
        let should_ignore_cursor = !mouse_over_widget;
        if should_ignore_cursor != is_cursor_ignored {
            // Check the flag with read lock acquired to avoid racing with the
            // writers on setting `ignore_cursor_events`
            let state = canvas.state::<CanvasImodeState>();
            let _guard = match state.lock.try_read() {
                Ok(guard) => guard,
                Err(_) => return, // Avoid blocking
            };

            if !LISTENING_MOUSEMOVE.load(Ordering::Acquire) {
                return;
            }
            is_cursor_ignored = should_ignore_cursor;
            if let Err(e) = canvas.set_ignore_cursor_events(should_ignore_cursor) {
                eprintln!("Failed to set cursor events state: {e}");
            }
        }
    })?;

    Ok(())
}
