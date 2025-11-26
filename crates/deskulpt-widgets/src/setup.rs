//! Setup state of frontend windows.
//!
//! TODO: Move this to a separate high-level crate that depends on this crate
//! because it is not specific to widgets.

use std::sync::atomic::{AtomicU8, Ordering};

use bitflags::bitflags;
use deskulpt_common::window::DeskulptWindow;

bitflags! {
  /// Flags representing the setup state of frontend windows.
  struct SetupFlags: u8 {
    const CANVAS = 1 << 0;
    const MANAGER = 1 << 1;
  }
}

impl From<DeskulptWindow> for SetupFlags {
    fn from(window: DeskulptWindow) -> Self {
        match window {
            DeskulptWindow::Canvas => SetupFlags::CANVAS,
            DeskulptWindow::Manager => SetupFlags::MANAGER,
        }
    }
}

/// Setup state of frontend windows.
///
/// This is essentially an atomic bitmask corresponding to [`SetupFlags`]. Each
/// set bit means that the corresponding window has completed setup.
#[derive(Default)]
pub struct SetupState(AtomicU8);

impl SetupState {
    /// Mark a window as having completed setup.
    ///
    /// Returns `true` if all windows have completed setup after this call,
    /// AND this is the first time all windows are complete (i.e., this call
    /// was the one that completed the set).
    pub fn complete(&self, window: DeskulptWindow) -> bool {
        let flag: SetupFlags = window.into();
        // Set the corresponding bit and retrieve the previous state; we only
        // need AcqRel not SeqCst because we don't care about total ordering
        let prev = self.0.fetch_or(flag.bits(), Ordering::AcqRel);
        let prev_flags = SetupFlags::from_bits_truncate(prev);
        let was_already_complete = prev_flags.is_all();
        let current = prev_flags | flag;
        let is_now_complete = current.is_all();
        // Only return true if:
        // 1. All windows are now complete
        // 2. They weren't all complete before (this call completed the set)
        is_now_complete && !was_already_complete
    }
}
