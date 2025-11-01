//! State management for application setup.

use std::sync::atomic::{AtomicU8, Ordering};

use bitflags::bitflags;
use deskulpt_common::window::DeskulptWindow;
use tauri::{App, AppHandle, Manager, Runtime};

bitflags! {
  /// Flags representing the setup completion state of different windows.
  struct Flags: u8 {
    const CANVAS = 1 << 0;
    const MANAGER = 1 << 1;
  }
}

/// Managed state for application setup.
#[derive(Default)]
pub struct SetupState(
    /// Bitmask corresponding to [`Flags`].
    ///
    /// Each set bit means that the corresponding window has completed setup.
    AtomicU8,
);

/// Extension trait for operations related to application setup.
pub trait SetupStateExt<R: Runtime>: Manager<R> {
    /// Mark a window as having completed setup.
    ///
    /// Returns `true` if all windows have completed setup after this call.
    fn complete_setup(&self, window: DeskulptWindow) -> bool {
        let state = self.state::<SetupState>();
        let bit = match window {
            DeskulptWindow::Canvas => Flags::CANVAS,
            DeskulptWindow::Manager => Flags::MANAGER,
        };

        // Set the corresponding bit and retrieve the previous state; we only
        // need AcqRel not SeqCst because we don't care about total ordering
        let prev = state.0.fetch_or(bit.bits(), Ordering::AcqRel);
        let current = Flags::from_bits_truncate(prev) | bit;
        current.is_all()
    }
}

impl<R: Runtime> SetupStateExt<R> for App<R> {}
impl<R: Runtime> SetupStateExt<R> for AppHandle<R> {}
