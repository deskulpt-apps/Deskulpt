#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

//! Deskulpt Plugin SDK
//!
//! This crate provides the necessary APIs and macros for building Deskulpt
//! plugins as dynamic libraries with C ABI compatibility.

mod abi;
mod engine;
mod plugin;
mod registry;

pub use abi::*;
// Re-export commonly used types
pub use anyhow::{self, Result};
pub use engine::EngineInterface;
pub use plugin::{Plugin, PluginCommand, PluginInfo, TypedPluginCommand};
pub use registry::PluginRegistry;
pub use serde_json;

/// Initialize a plugin with the given engine callbacks.
///
/// This function should be called by the plugin's `plugin_init` export.
/// It sets up the engine interface and returns plugin information.
pub fn init_plugin<P: Plugin + 'static>(
    plugin: P,
    engine_callbacks: EngineCallbacks,
) -> Result<PluginInfo> {
    let mut registry = PluginRegistry::new();

    // Register the plugin
    let plugin_name = plugin.name().to_string();
    let plugin_version = plugin.version();
    let commands = plugin.commands();

    registry.register_plugin(plugin_name.clone(), Box::new(plugin))?;

    // Store the registry globally
    unsafe {
        PLUGIN_REGISTRY = Some(registry);
        ENGINE_CALLBACKS = Some(engine_callbacks);
    }

    Ok(PluginInfo {
        name: plugin_name,
        version: plugin_version,
        commands: commands
            .into_iter()
            .map(|cmd| cmd.name().to_string())
            .collect(),
    })
}

/// Call a plugin command.
///
/// This function should be called by the plugin's `plugin_call_command` export.
pub fn call_command(command_name: &str, widget_id: &str, payload: &str) -> Result<String> {
    let registry = unsafe {
        PLUGIN_REGISTRY
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Plugin not initialized"))?
    };

    let engine_callbacks = unsafe {
        ENGINE_CALLBACKS
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Engine callbacks not available"))?
    };

    let engine = EngineInterface::new(*engine_callbacks);

    let result = registry.call_command(command_name, widget_id, &engine, payload)?;
    Ok(result)
}

/// Cleanup plugin resources.
///
/// This function should be called by the plugin's `plugin_destroy` export.
pub fn destroy_plugin() {
    unsafe {
        PLUGIN_REGISTRY = None;
        ENGINE_CALLBACKS = None;
    }
}

// Global state for the plugin
static mut PLUGIN_REGISTRY: Option<PluginRegistry> = None;
static mut ENGINE_CALLBACKS: Option<EngineCallbacks> = None;

/// Convenience macro for implementing the required C ABI exports for a plugin.
///
/// This macro generates the necessary `extern "C"` functions that the plugin
/// loader expects.
///
/// # Example
///
/// ```rust
/// use deskulpt_plugin::{implement_plugin, Plugin, PluginCommand};
///
/// struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn name(&self) -> &str { "my-plugin" }
///     fn version(&self) -> String { "1.0.0".to_string() }
///     fn commands(&self) -> Vec<Box<dyn PluginCommand>> { vec![] }
/// }
///
/// implement_plugin!(MyPlugin);
/// ```
#[macro_export]
macro_rules! implement_plugin {
    ($plugin_type:ty) => {
        use std::ffi::{c_char, CStr, CString};
        use std::ptr;

        use $crate::{EngineCallbacks, PluginInfo};

        #[no_mangle]
        pub extern "C" fn plugin_init(
            engine_callbacks: $crate::EngineCallbacks,
            info_out: *mut $crate::PluginInfo,
        ) -> i32 {
            let plugin = <$plugin_type>::default();

            match $crate::init_plugin(plugin, engine_callbacks) {
                Ok(info) => {
                    if !info_out.is_null() {
                        unsafe {
                            *info_out = info;
                        }
                    }
                    0 // Success
                },
                Err(_) => -1, // Error
            }
        }

        #[no_mangle]
        pub extern "C" fn plugin_call_command(
            command_name: *const c_char,
            widget_id: *const c_char,
            payload: *const c_char,
            result_out: *mut *mut c_char,
        ) -> i32 {
            if command_name.is_null() || widget_id.is_null() || payload.is_null() {
                return -1;
            }

            let command_name = unsafe {
                match CStr::from_ptr(command_name).to_str() {
                    Ok(s) => s,
                    Err(_) => return -1,
                }
            };

            let widget_id = unsafe {
                match CStr::from_ptr(widget_id).to_str() {
                    Ok(s) => s,
                    Err(_) => return -1,
                }
            };

            let payload = unsafe {
                match CStr::from_ptr(payload).to_str() {
                    Ok(s) => s,
                    Err(_) => return -1,
                }
            };

            match $crate::call_command(command_name, widget_id, payload) {
                Ok(result) => {
                    if !result_out.is_null() {
                        match CString::new(result) {
                            Ok(c_string) => {
                                unsafe {
                                    *result_out = c_string.into_raw();
                                }
                                0 // Success
                            },
                            Err(_) => -1,
                        }
                    } else {
                        0
                    }
                },
                Err(_) => -1,
            }
        }

        #[no_mangle]
        pub extern "C" fn plugin_destroy() {
            $crate::destroy_plugin();
        }

        #[no_mangle]
        pub extern "C" fn plugin_free_string(ptr: *mut c_char) {
            if !ptr.is_null() {
                unsafe {
                    let _ = CString::from_raw(ptr);
                }
            }
        }
    };
}

/// Register commands in a Deskulpt plugin.
///
/// This macro provides an automatic implementation of the [`Plugin::commands`]
/// method. Each registered command must implement the [`PluginCommand`] trait.
///
/// ### Example
///
/// ```no_run
/// use deskulpt_plugin::{register_commands, Plugin};
///
/// struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     register_commands![/* List of commands to register */];
/// }
/// ```
#[macro_export]
macro_rules! register_commands {
    ($($command:path),* $(,)?) => {
        fn commands(&self) -> Vec<Box<dyn $crate::PluginCommand>> {
            vec![$(Box::new($command),)*]
        }
    };
}
