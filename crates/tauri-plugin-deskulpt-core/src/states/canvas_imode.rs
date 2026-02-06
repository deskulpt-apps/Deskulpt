//! State management for canvas interaction mode.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use anyhow::Result;
use deskulpt_common::event::Event;
use deskulpt_common::window::DeskulptWindow;
use parking_lot::RwLock;
use seqlock::SeqLock;
use tauri::{App, AppHandle, Manager, PhysicalPosition, Runtime, WebviewWindow};
use tauri_plugin_deskulpt_settings::SettingsExt;
use tauri_plugin_deskulpt_settings::model::{CanvasImode, SettingsPatch};
use tauri_plugin_deskulpt_widgets::WidgetsExt;
use tracing::error;

use crate::events::ShowToastEvent;

/// Layout information of the canvas.
#[derive(Copy, Clone)]
struct CanvasLayout {
    /// Physical x-coordinate.
    x: f64,
    /// Physical y-coordinate.
    y: f64,
    /// Inverse of the scale factor.
    inv_scale: f64,
}

/// Managed state for canvas interaction mode.
struct CanvasImodeState {
    /// Lock for serializing `set_ignore_cursor_events` calls.
    lock: RwLock<()>,
    /// Layout information of the canvas.
    ///
    /// We use [`SeqLock`] here for low-overhead and lock-free reads in the
    /// global mousemove event listener which cannot afford blocking, thanks to
    /// the fact that [`CanvasLayout`] is [`Copy`]. Writers must be rare, which
    /// is the case here since they only happen when the canvas is moved or
    /// rescaled, mostly on startup.
    layout: SeqLock<CanvasLayout>,
}

/// Whether the global mousemove listener is enabled.
static LISTENING_MOUSEMOVE: AtomicBool = AtomicBool::new(false);

/// Extension trait for operations on canvas interaction mode.
pub trait CanvasImodeStateExt<R: Runtime>: Manager<R> + SettingsExt<R> {
    /// Initialize state management for canvas interaction mode.
    ///
    /// This will also hook into settings changes and global mousemove events
    /// and update the canvas interaction mode accordingly.
    fn manage_canvas_imode(&self) -> Result<()> {
        let canvas = DeskulptWindow::Canvas.webview_window(self)?;
        let canvas_position = canvas.inner_position()?;
        let canvas_layout = CanvasLayout {
            x: canvas_position.x as f64,
            y: canvas_position.y as f64,
            inv_scale: 1.0 / canvas.scale_factor()?,
        };
        self.manage(CanvasImodeState {
            lock: RwLock::new(()),
            layout: SeqLock::new(canvas_layout),
        });

        let canvas_cloned = canvas.clone();
        std::thread::spawn(move || {
            // Delay the start of mousemove listener to avoid interfering with
            // canvas initialization, which is in most cases the heaviest period
            // of writes to states that the mousemove listener may read; users
            // commonly won't notice such delay because window creation and
            // widgets rendering also take time.
            std::thread::sleep(Duration::from_secs(1));

            if let Err(e) = listen_to_mousemove(canvas_cloned) {
                eprintln!("Failed to listen to global mousemove events: {}", e);
            }
        });

        if self.settings().read().canvas_imode == CanvasImode::Auto {
            LISTENING_MOUSEMOVE.store(true, Ordering::Release);
        }

        self.settings().on_canvas_imode_change(move |_, new| {
            if let Err(e) = on_new_canvas_imode(&canvas, new) {
                error!("Failed to update canvas interaction mode: {}", e);
            }
        });

        Ok(())
    }

    /// Set the position of the canvas.
    ///
    /// This should be called whenever the canvas is moved.
    fn set_canvas_position(&self, position: &PhysicalPosition<i32>) {
        let state = self.state::<CanvasImodeState>();
        let mut layout = state.layout.lock_write();
        layout.x = position.x as f64;
        layout.y = position.y as f64;
    }

    /// Set the scale factor of the canvas.
    ///
    /// This should be called whenever the canvas scale factor changes.
    fn set_canvas_scale_factor(&self, scale_factor: f64) {
        let state = self.state::<CanvasImodeState>();
        let mut layout = state.layout.lock_write();
        layout.inv_scale = 1.0 / scale_factor;
    }

    /// Toggle the interaction mode of the canvas.
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
/// This updates the canvas click-through state and the mousemove event
/// listener's behavior according to the given mode. It also emits a toast
/// notification to the canvas, but failure to do so is non-fatal and will not
/// result in an error.
fn on_new_canvas_imode<R: Runtime>(canvas: &WebviewWindow<R>, mode: &CanvasImode) -> Result<()> {
    match mode {
        CanvasImode::Auto => {
            LISTENING_MOUSEMOVE.store(true, Ordering::Release);
        },
        CanvasImode::Sink | CanvasImode::Float => {
            // Set the flag with write lock acquired to avoid racing with the
            // mousemove hook on setting `ignore_cursor_events`
            let state = canvas.state::<CanvasImodeState>();
            let _guard = state.lock.write();
            LISTENING_MOUSEMOVE.store(false, Ordering::Release);
            canvas.set_ignore_cursor_events(*mode == CanvasImode::Sink)?;
        },
    }

    if let Err(e) = ShowToastEvent::Success(format!("Canvas interaction mode: {mode:?}"))
        .emit_to(canvas, DeskulptWindow::Canvas)
    {
        error!("Failed to emit ShowToastEvent to canvas: {}", e);
    }

    Ok(())
}

/// Global mousemove event listener.
///
/// If the cheap check on [`LISTENING_MOUSEMOVE`] gives false, the hook will
/// short-circuit immediately, effectively disabling the listener. Otherwise,
/// it will check whether the mouse is over any widget in the canvas. If so, the
/// canvas will accept cursor events; otherwise, it will ignore them.
fn listen_to_mousemove<R: Runtime>(canvas: WebviewWindow<R>) -> Result<()> {
    let mut is_cursor_ignored = true;

    global_mousemove::listen(move |event| {
        if !LISTENING_MOUSEMOVE.load(Ordering::Acquire) {
            return;
        }

        let state = canvas.state::<CanvasImodeState>();
        let canvas_layout = state.layout.read();

        let global_mousemove::MouseMoveEvent { x, y } = event;

        // For macOS, mousemove coordinates are in logical coordinates, so
        // only canvas physical position needs to be scaled
        #[cfg(target_os = "macos")]
        let scaled_x = x - canvas_layout.x * canvas_layout.inv_scale;
        #[cfg(target_os = "macos")]
        let scaled_y = y - canvas_layout.y * canvas_layout.inv_scale;

        // For other platforms, mousemove coordinates are in physical
        // coordinates, so they need to be scaled together with canvas position
        #[cfg(not(target_os = "macos"))]
        let scaled_x = (x - canvas_layout.x) * canvas_layout.inv_scale;
        #[cfg(not(target_os = "macos"))]
        let scaled_y = (y - canvas_layout.y) * canvas_layout.inv_scale;

        let Some(mouse_over_widget) = canvas.widgets().try_covers_point(scaled_x, scaled_y) else {
            return; // Avoid blocking
        };

        // Avoid redundant calls by checking if the state has really changed
        let should_ignore_cursor = !mouse_over_widget;
        if should_ignore_cursor != is_cursor_ignored {
            // Check the flag with read lock acquired to avoid racing with the
            // writers on setting `ignore_cursor_events`
            let state = canvas.state::<CanvasImodeState>();
            let _guard = match state.lock.try_read() {
                Some(guard) => guard,
                None => return, // Avoid blocking
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
