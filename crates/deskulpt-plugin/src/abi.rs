//! C ABI definitions for plugin interface.

use std::ffi::c_char;

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

/// Engine callback function for getting widget directory.
///
/// # Arguments
/// * `widget_id` - Widget ID as null-terminated C string
/// * `path_out` - Output buffer for the path (caller must free with
///   plugin_free_string)
///
/// # Returns
/// * 0 on success, -1 on error
pub type WidgetDirCallback =
    unsafe extern "C" fn(widget_id: *const c_char, path_out: *mut *mut c_char) -> i32;

/// Engine callback function for logging.
///
/// # Arguments
/// * `level` - Log level (0=error, 1=warn, 2=info, 3=debug, 4=trace)
/// * `message` - Log message as null-terminated C string
pub type LogCallback = unsafe extern "C" fn(level: i32, message: *const c_char);

/// Engine callbacks provided by the core to plugins.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EngineCallbacks {
    /// Get widget directory callback
    pub widget_dir: WidgetDirCallback,
    /// Log callback
    pub log: LogCallback,
}

/// Plugin initialization function signature.
///
/// This function must be exported by every plugin as `plugin_init`.
///
/// # Arguments
/// * `engine_callbacks` - Callbacks provided by the engine
/// * `info_out` - Output parameter for plugin information
///
/// # Returns
/// * 0 on success, negative value on error
pub type PluginInitFn =
    unsafe extern "C" fn(engine_callbacks: EngineCallbacks, info_out: *mut PluginInfo) -> i32;

/// Plugin command call function signature.
///
/// This function must be exported by every plugin as `plugin_call_command`.
///
/// # Arguments
/// * `command_name` - Name of the command to call
/// * `widget_id` - ID of the widget making the call
/// * `payload` - JSON payload as null-terminated C string
/// * `result_out` - Output parameter for result JSON (caller must free with
///   plugin_free_string)
///
/// # Returns
/// * 0 on success, negative value on error
pub type PluginCallCommandFn = unsafe extern "C" fn(
    command_name: *const c_char,
    widget_id: *const c_char,
    payload: *const c_char,
    result_out: *mut *mut c_char,
) -> i32;

/// Plugin destruction function signature.
///
/// This function must be exported by every plugin as `plugin_destroy`.
pub type PluginDestroyFn = unsafe extern "C" fn();

/// String memory cleanup function signature.
///
/// This function must be exported by every plugin as `plugin_free_string`.
///
/// # Arguments
/// * `ptr` - Pointer to string allocated by the plugin
pub type PluginFreeStringFn = unsafe extern "C" fn(ptr: *mut c_char);
