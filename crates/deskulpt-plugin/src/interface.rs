//! Interaction interface.

use std::path::PathBuf;

/// The interface for interacting with the Deskulpt engine (🚧 TODO 🚧).
///
/// ### 🚧 TODO 🚧
///
/// This is a temporary implementation that directly takes the necessary
/// functions for the engine interface from the Deskulpt core, because the
/// plugins currently run in the same process as the core. The final
/// implementation should not require this and should use IPC for communication.
/// This struct may need to hold the IPC channel, etc. instead.
pub struct EngineInterface {
    #[allow(clippy::type_complexity)]
    widget_dir_fn: Box<dyn Fn(&str) -> PathBuf>,
}

impl EngineInterface {
    /// Create a new engine interface instance.
    pub fn new(widget_dir_fn: impl Fn(&str) -> PathBuf + 'static) -> Self {
        Self {
            widget_dir_fn: Box::new(widget_dir_fn),
        }
    }

    /// Get the directory of a widget (🚧 TODO 🚧).
    ///
    /// # 🚧 TODO 🚧
    ///
    /// This method is a temporary implementation. The final implementation
    /// should use IPC to communicate with the Deskulpt core to get the widget
    /// directory.
    pub fn widget_dir(&self, id: &str) -> PathBuf {
        (self.widget_dir_fn)(id)
    }
}
