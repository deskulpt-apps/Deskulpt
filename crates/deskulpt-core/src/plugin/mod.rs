//! Plugin management system for loading and managing DLL-based plugins.

mod loader;
mod manager;

use std::ffi::{c_char, CStr, CString};
use std::path::PathBuf;

use anyhow::Result;
pub use loader::{LoadedPlugin, PluginLoader};
pub use manager::PluginManager;

/// Engine callbacks that are provided to plugins.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EngineCallbacks {
    /// Get widget directory callback
    pub widget_dir:
        unsafe extern "C" fn(widget_id: *const c_char, path_out: *mut *mut c_char) -> i32,
    /// Log callback
    pub log: unsafe extern "C" fn(level: i32, message: *const c_char),
}

/// Plugin information returned during initialization.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PluginInfo {
    /// Plugin name (owned string)
    pub name: String,
    /// Plugin version (owned string)
    pub version: String,
    /// List of command names (owned strings)
    pub commands: Vec<String>,
}

impl EngineCallbacks {
    /// Create engine callbacks with the provided widget directory function.
    pub fn new<F>(widget_dir_fn: F) -> Self
    where
        F: Fn(&str) -> Result<PathBuf> + Send + Sync + 'static,
    {
        // Store the closure in a thread-local static
        use std::sync::{Arc, Mutex};

        static WIDGET_DIR_FN: std::sync::OnceLock<
            Arc<Mutex<Box<dyn Fn(&str) -> Result<PathBuf> + Send + Sync>>>,
        > = std::sync::OnceLock::new();

        let _ = WIDGET_DIR_FN.set(Arc::new(Mutex::new(Box::new(widget_dir_fn))));

        Self {
            widget_dir: widget_dir_callback,
            log: log_callback,
        }
    }
}

/// C callback for getting widget directory.
unsafe extern "C" fn widget_dir_callback(
    widget_id: *const c_char,
    path_out: *mut *mut c_char,
) -> i32 {
    if widget_id.is_null() || path_out.is_null() {
        return -1;
    }

    let widget_id_str = match CStr::from_ptr(widget_id).to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    // Access the stored function
    use std::sync::{Arc, Mutex};

    static WIDGET_DIR_FN: std::sync::OnceLock<
        Arc<Mutex<Box<dyn Fn(&str) -> Result<PathBuf> + Send + Sync>>>,
    > = std::sync::OnceLock::new();

    let result = if let Some(func_arc) = WIDGET_DIR_FN.get() {
        if let Ok(func) = func_arc.lock() {
            func(widget_id_str)
        } else {
            return -1;
        }
    } else {
        return -1;
    };

    match result {
        Ok(path) => match CString::new(path.to_string_lossy().to_string()) {
            Ok(c_string) => {
                *path_out = c_string.into_raw();
                0
            },
            Err(_) => -1,
        },
        Err(_) => -1,
    }
}

/// C callback for logging.
unsafe extern "C" fn log_callback(level: i32, message: *const c_char) {
    if message.is_null() {
        return;
    }

    let message_str = match CStr::from_ptr(message).to_str() {
        Ok(s) => s,
        Err(_) => return,
    };

    match level {
        0 => log::error!("{}", message_str),
        1 => log::warn!("{}", message_str),
        2 => log::info!("{}", message_str),
        3 => log::debug!("{}", message_str),
        4 => log::trace!("{}", message_str),
        _ => log::info!("{}", message_str),
    }
}
