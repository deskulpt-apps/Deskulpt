//! State management for canvas interaction mode.

use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::Result;
use deskulpt_common::event::Event;
use deskulpt_common::window::DeskulptWindow;
use deskulpt_settings::{CanvasImode, SettingsExt, SettingsPatch};
use tauri::{App, AppHandle, Emitter, Manager, Runtime, WebviewWindow};

use crate::events::ShowToastEvent;

/// The mousemove listener state.
///
/// This is represented as an [`AtomicU64`], where the first bit indicates
/// whether the listener is enabled (1) or not (0), and the rest of the bits
/// represent an [`u64`] epoch counter.
struct ListenerState(AtomicU64);

impl ListenerState {
    /// Get the current enabled state and epoch.
    ///
    /// This method uses the acquire memory ordering to "subscribe" to updates
    /// from [`Self::set_enabled`], synchronizing with its release memory
    /// ordering. This guarantees that the method always reads the latest value
    /// written by [`Self::set_enabled`], and that no memory operations *after*
    /// this load can be reordered to happen *before* it.
    fn get(&self) -> (bool, u64) {
        let v = self.0.load(Ordering::Acquire);
        ((v & 1) == 1, v >> 1)
    }

    /// Set the enabled state and increment the epoch.
    ///
    /// This method is thread-safe using an atomic compare-and-swap operation.
    /// It uses the release memory ordering to "publish" the updated state to
    /// all [`Self::get`] "subscribers", ensuring they will see this new value.
    fn set_enabled(&self, enabled: bool) {
        let _ = self
            .0
            // On top of release memory ordering for "set", it is typical to add
            // "acquire" semantics for both "set" and "fetch" in CAS operations
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |old| {
                let new_epoch = (old >> 1).wrapping_add(1);
                Some((new_epoch << 1) | (enabled as u64))
            });
    }
}

/// Global listener state for mousemove events.
static LISTENER_STATE: ListenerState = ListenerState(AtomicU64::new(0));

/// Extension trait for operations on canvas interaction mode.
pub trait CanvasImodeExt<R: Runtime>: Manager<R> + Emitter<R> + SettingsExt<R> {
    /// Initialize management for canvas interaction mode.
    ///
    /// This will hook into settings changes and global mousemove events and
    /// update the canvas window's interaction mode accordingly.
    fn init_canvas_imode(&self) -> Result<()> {
        let canvas = DeskulptWindow::Canvas.webview_window(self)?;
        let canvas_cloned = canvas.clone();

        std::thread::spawn(move || {
            if let Err(e) = listen_to_mousemove(canvas) {
                eprintln!("Failed to listen to global mousemove events: {}", e);
            }
        });

        if self.settings().read().canvas_imode == CanvasImode::Auto {
            LISTENER_STATE.set_enabled(true);
        }

        self.settings().on_canvas_imode_change(move |_, new| {
            if let Err(e) = on_new_canvas_imode(&canvas_cloned, new) {
                eprintln!("Failed to update canvas interaction mode: {}", e);
            }
        });

        Ok(())
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

impl<R: Runtime> CanvasImodeExt<R> for App<R> {}
impl<R: Runtime> CanvasImodeExt<R> for AppHandle<R> {}

/// Handler for canvas interaction mode changes.
///
/// This updates the canvas window's click-through state and the menu item
/// text. It also emits a toast notification to the canvas window, but failure
/// to do so is non-fatal and will not result in an error.
fn on_new_canvas_imode<R: Runtime>(canvas: &WebviewWindow<R>, mode: &CanvasImode) -> Result<()> {
    match mode {
        CanvasImode::Auto => {
            LISTENER_STATE.set_enabled(true);
        },
        CanvasImode::Sink => {
            LISTENER_STATE.set_enabled(false);
            canvas.set_ignore_cursor_events(true)?;
        },
        CanvasImode::Float => {
            LISTENER_STATE.set_enabled(false);
            canvas.set_ignore_cursor_events(false)?;
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
/// If [`LISTENER_STATE`] shows that the listener is disabled, this will short-
/// circuit immediately. Otherwise, it checks whether the mouse is over any
/// widget in the canvas window. If so, the canvas will accept cursor events;
/// otherwise, it will ignore them.
fn listen_to_mousemove<R: Runtime>(canvas: WebviewWindow<R>) -> Result<()> {
    let scale_factor = canvas.scale_factor()?;
    let canvas_position = canvas.inner_position()?;
    let canvas_x = canvas_position.x as f64;
    let canvas_y = canvas_position.y as f64;

    let mut is_cursor_ignored = true;

    global_mousemove::listen(move |event| {
        let (enabled0, epoch0) = LISTENER_STATE.get();
        if !enabled0 {
            return;
        }

        let global_mousemove::MouseMoveEvent { x, y } = event;
        let scaled_x = (x - canvas_x) / scale_factor;
        let scaled_y = (y - canvas_y) / scale_factor;

        let settings = canvas.settings().read();
        let mouse_over_widget = settings.widgets.values().any(|widget| {
            scaled_x >= widget.x as f64
                && scaled_x < widget.x as f64 + widget.width as f64
                && scaled_y >= widget.y as f64
                && scaled_y < widget.y as f64 + widget.height as f64
        });

        // Avoid redundant calls by checking if the state has really changed
        let should_ignore_cursor = !mouse_over_widget;
        if should_ignore_cursor != is_cursor_ignored {
            // Double-check before doing actual work: if the listener has been
            // disabled while we were processing, or if the epoch has changed
            // which means that some other update has been published, we should
            // abort to prevent a "stale" event applying incorrect effect
            let (enabled1, epoch1) = LISTENER_STATE.get();
            if !enabled1 || epoch0 != epoch1 {
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
