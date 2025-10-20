//! State management for application setup.

use std::sync::atomic::{AtomicBool, Ordering};

use deskulpt_common::window::DeskulptWindow;
use tauri::{App, AppHandle, Emitter, Manager, Runtime};

/// Managed state for application setup.
#[derive(Default)]
struct SetupState {
    canvas: AtomicBool,
    manager: AtomicBool,
}

/// Extension trait for operations related to the initial render.
pub trait SetupStateExt<R: Runtime>: Manager<R> + Emitter<R> {
    /// Initialize state management for the initial render.
    fn manage_setup(&self) {
        self.manage(SetupState::default());
    }

    /// Mark a window as having completed setup.
    ///
    /// This method returns `true` if both windows have now completed setup
    /// **for the first time**.
    fn complete_setup(&self, window: DeskulptWindow) -> bool {
        let state = self.state::<SetupState>();
        let (flag, other_complete) = match window {
            DeskulptWindow::Canvas => (&state.canvas, state.manager.load(Ordering::SeqCst)),
            DeskulptWindow::Manager => (&state.manager, state.canvas.load(Ordering::SeqCst)),
        };
        let prev_complete = flag.swap(true, Ordering::SeqCst);
        !prev_complete && other_complete
    }
}

impl<R: Runtime> SetupStateExt<R> for App<R> {}
impl<R: Runtime> SetupStateExt<R> for AppHandle<R> {}
