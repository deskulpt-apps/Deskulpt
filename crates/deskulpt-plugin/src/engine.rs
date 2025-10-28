//! Engine interface for plugins to interact with the Deskulpt core.

use std::ffi::{c_char, CStr, CString};
use std::path::PathBuf;

use anyhow::{anyhow, Result};

use crate::abi::EngineCallbacks;

/// The interface for interacting with the Deskulpt engine.
///
/// This provides a safe Rust wrapper around the C ABI callbacks provided by the
/// engine.
pub struct EngineInterface {
    callbacks: EngineCallbacks,
}

impl EngineInterface {
    /// Create a new engine interface with the provided callbacks.
    pub(crate) fn new(callbacks: EngineCallbacks) -> Self {
        Self { callbacks }
    }

    /// Get the directory of a widget.
    ///
    /// # Arguments
    /// * `widget_id` - The ID of the widget
    ///
    /// # Returns
    /// * `Ok(PathBuf)` - The path to the widget directory
    /// * `Err(anyhow::Error)` - If the widget directory could not be retrieved
    pub fn widget_dir(&self, widget_id: &str) -> Result<PathBuf> {
        let widget_id_c = CString::new(widget_id)
            .map_err(|_| anyhow!("Invalid widget ID: contains null bytes"))?;

        let mut path_ptr: *mut c_char = std::ptr::null_mut();

        let result = unsafe { (self.callbacks.widget_dir)(widget_id_c.as_ptr(), &mut path_ptr) };

        if result != 0 {
            return Err(anyhow!(
                "Failed to get widget directory for ID: {}",
                widget_id
            ));
        }

        if path_ptr.is_null() {
            return Err(anyhow!(
                "Engine returned null path for widget ID: {}",
                widget_id
            ));
        }

        let path_str = unsafe {
            let c_str = CStr::from_ptr(path_ptr);
            let result = c_str
                .to_str()
                .map_err(|_| anyhow!("Invalid UTF-8 in widget directory path"))?
                .to_string();

            // Free the string allocated by the engine
            libc::free(path_ptr as *mut libc::c_void);

            result
        };

        Ok(PathBuf::from(path_str))
    }

    /// Log a message through the engine.
    ///
    /// # Arguments
    /// * `level` - The log level (Error, Warn, Info, Debug, Trace)
    /// * `message` - The message to log
    pub fn log(&self, level: LogLevel, message: &str) {
        if let Ok(message_c) = CString::new(message) {
            unsafe {
                (self.callbacks.log)(level as i32, message_c.as_ptr());
            }
        }
    }

    /// Log an error message.
    pub fn log_error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    /// Log a warning message.
    pub fn log_warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    /// Log an info message.
    pub fn log_info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    /// Log a debug message.
    pub fn log_debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    /// Log a trace message.
    pub fn log_trace(&self, message: &str) {
        self.log(LogLevel::Trace, message);
    }
}

/// Log levels for engine logging.
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    /// Error level
    Error = 0,
    /// Warning level
    Warn = 1,
    /// Info level
    Info = 2,
    /// Debug level
    Debug = 3,
    /// Trace level
    Trace = 4,
}
