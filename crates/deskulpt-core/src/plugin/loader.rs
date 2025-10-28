//! Plugin loader for loading individual DLL plugins.

use std::ffi::{c_char, CStr, CString};
use std::path::Path;

use anyhow::{anyhow, Result};
use libloading::{Library, Symbol};

use super::{EngineCallbacks, PluginInfo};

/// Function signatures for plugin exports.
type PluginInitFn =
    unsafe extern "C" fn(engine_callbacks: EngineCallbacks, info_out: *mut PluginInfo) -> i32;

type PluginCallCommandFn = unsafe extern "C" fn(
    command_name: *const c_char,
    widget_id: *const c_char,
    payload: *const c_char,
    result_out: *mut *mut c_char,
) -> i32;

type PluginDestroyFn = unsafe extern "C" fn();

type PluginFreeStringFn = unsafe extern "C" fn(ptr: *mut c_char);

/// A loaded plugin instance.
pub struct LoadedPlugin {
    /// The loaded library
    #[allow(dead_code)]
    library: Library,
    /// Plugin information
    info: PluginInfo,
    /// Function to call commands
    call_command: Symbol<'static, PluginCallCommandFn>,
    /// Function to destroy the plugin
    destroy: Symbol<'static, PluginDestroyFn>,
    /// Function to free strings allocated by the plugin
    free_string: Symbol<'static, PluginFreeStringFn>,
}

impl LoadedPlugin {
    /// Get the plugin information.
    pub fn info(&self) -> &PluginInfo {
        &self.info
    }

    /// Call a command in the plugin.
    ///
    /// # Arguments
    /// * `command_name` - The name of the command to call
    /// * `widget_id` - The ID of the widget making the call
    /// * `payload` - The JSON payload as a string
    ///
    /// # Returns
    /// * `Ok(String)` - The JSON result from the command
    /// * `Err(anyhow::Error)` - If the command fails
    pub fn call_command(
        &self,
        command_name: &str,
        widget_id: &str,
        payload: &str,
    ) -> Result<String> {
        let command_name_c = CString::new(command_name)
            .map_err(|_| anyhow!("Invalid command name: contains null bytes"))?;

        let widget_id_c = CString::new(widget_id)
            .map_err(|_| anyhow!("Invalid widget ID: contains null bytes"))?;

        let payload_c =
            CString::new(payload).map_err(|_| anyhow!("Invalid payload: contains null bytes"))?;

        let mut result_ptr: *mut c_char = std::ptr::null_mut();

        let return_code = unsafe {
            (self.call_command)(
                command_name_c.as_ptr(),
                widget_id_c.as_ptr(),
                payload_c.as_ptr(),
                &mut result_ptr,
            )
        };

        if return_code != 0 {
            return Err(anyhow!(
                "Plugin command '{}' failed with code: {}",
                command_name,
                return_code
            ));
        }

        if result_ptr.is_null() {
            return Err(anyhow!("Plugin returned null result"));
        }

        let result_string = unsafe {
            let c_str = CStr::from_ptr(result_ptr);
            let result = c_str
                .to_str()
                .map_err(|_| anyhow!("Invalid UTF-8 in plugin result"))?
                .to_string();

            // Free the string allocated by the plugin
            (self.free_string)(result_ptr);

            result
        };

        Ok(result_string)
    }

    /// Check if the plugin has a specific command.
    pub fn has_command(&self, command_name: &str) -> bool {
        self.info.commands.iter().any(|cmd| cmd == command_name)
    }

    /// Get all command names provided by this plugin.
    pub fn command_names(&self) -> &[String] {
        &self.info.commands
    }
}

impl Drop for LoadedPlugin {
    fn drop(&mut self) {
        // Call the plugin's destroy function
        unsafe {
            (self.destroy)();
        }
    }
}

/// Plugin loader for loading DLL-based plugins.
pub struct PluginLoader;

impl PluginLoader {
    /// Load a plugin from the specified path.
    ///
    /// # Arguments
    /// * `path` - Path to the plugin DLL
    /// * `engine_callbacks` - Engine callbacks to provide to the plugin
    ///
    /// # Returns
    /// * `Ok(LoadedPlugin)` - If the plugin was loaded successfully
    /// * `Err(anyhow::Error)` - If the plugin failed to load
    pub fn load_plugin<P: AsRef<Path>>(
        path: P,
        engine_callbacks: EngineCallbacks,
    ) -> Result<LoadedPlugin> {
        let path = path.as_ref();

        // Load the library
        let library = unsafe {
            Library::new(path)
                .map_err(|e| anyhow!("Failed to load plugin library '{}': {}", path.display(), e))?
        };

        // Get required function exports
        let init_fn: Symbol<PluginInitFn> = unsafe {
            library
                .get(b"plugin_init")
                .map_err(|e| anyhow!("Plugin missing 'plugin_init' export: {}", e))?
        };

        let call_command: Symbol<PluginCallCommandFn> = unsafe {
            library
                .get(b"plugin_call_command")
                .map_err(|e| anyhow!("Plugin missing 'plugin_call_command' export: {}", e))?
        };

        let destroy: Symbol<PluginDestroyFn> = unsafe {
            library
                .get(b"plugin_destroy")
                .map_err(|e| anyhow!("Plugin missing 'plugin_destroy' export: {}", e))?
        };

        let free_string: Symbol<PluginFreeStringFn> = unsafe {
            library
                .get(b"plugin_free_string")
                .map_err(|e| anyhow!("Plugin missing 'plugin_free_string' export: {}", e))?
        };

        // Initialize the plugin
        let mut plugin_info = PluginInfo {
            name: String::new(),
            version: String::new(),
            commands: Vec::new(),
        };

        let init_result = unsafe { init_fn(engine_callbacks, &mut plugin_info) };

        if init_result != 0 {
            return Err(anyhow!(
                "Plugin initialization failed with code: {}",
                init_result
            ));
        }

        // Convert symbols to 'static lifetime for storage
        let call_command = unsafe { std::mem::transmute(call_command) };
        let destroy = unsafe { std::mem::transmute(destroy) };
        let free_string = unsafe { std::mem::transmute(free_string) };

        Ok(LoadedPlugin {
            library,
            info: plugin_info,
            call_command,
            destroy,
            free_string,
        })
    }

    /// Check if a path appears to be a valid plugin DLL.
    ///
    /// This performs basic checks without actually loading the plugin.
    pub fn is_valid_plugin_path<P: AsRef<Path>>(path: P) -> bool {
        let path = path.as_ref();

        // Check if file exists and has the right extension
        if !path.exists() || !path.is_file() {
            return false;
        }

        // Check file extension
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("dll") if cfg!(windows) => true,
            Some("so") if cfg!(unix) => true,
            Some("dylib") if cfg!(target_os = "macos") => true,
            _ => false,
        }
    }
}
