//! Common utilities for Deskulpt events.

use anyhow::Result;
use serde::Serialize;
use tauri::{Emitter, Runtime};

use crate::window::DeskulptWindow;

/// Trait for Deskulpt events.
///
/// This trait should be derived using the [`derive@Event`] macro.
pub trait Event: Serialize {
    /// The name of the event.
    const NAME: &'static str;

    /// Emit the event to all target.
    fn emit<R, E>(&self, emitter: &E) -> Result<()>
    where
        R: Runtime,
        E: Emitter<R>,
    {
        emitter.emit(Self::NAME, self)?;
        Ok(())
    }

    /// Emit the event to the specified window.
    fn emit_to<R, E>(&self, emitter: &E, window: DeskulptWindow) -> Result<()>
    where
        R: Runtime,
        E: Emitter<R>,
    {
        emitter.emit_to(window, Self::NAME, self)?;
        Ok(())
    }
}

/// Derive the [`Event`] trait for a struct.
pub use deskulpt_macros::Event;
