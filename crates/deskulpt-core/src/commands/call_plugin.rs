use std::sync::OnceLock;

use anyhow::Result;
use tauri::{command, AppHandle, Manager, Runtime};
use tokio::sync::RwLock;

use super::error::CmdResult;
use crate::path::PathExt;
use crate::plugin::{EngineCallbacks, PluginManager};

/// Global plugin manager instance
static PLUGIN_MANAGER: OnceLock<RwLock<PluginManager>> = OnceLock::new();

/// Initialize the plugin manager with engine callbacks.
///
/// This should be called during application startup.
pub fn init_plugin_manager<R: Runtime>(app_handle: AppHandle<R>) -> Result<()> {
    let widget_dir_fn =
        move |widget_id: &str| -> Result<std::path::PathBuf> { app_handle.widget_dir(widget_id) };

    let engine_callbacks = EngineCallbacks::new(widget_dir_fn);
    let manager = PluginManager::new(engine_callbacks);

    PLUGIN_MANAGER
        .set(RwLock::new(manager))
        .map_err(|_| anyhow::anyhow!("Plugin manager already initialized"))?;

    Ok(())
}

/// Load plugins from the default plugin directory.
///
/// This should be called after initializing the plugin manager.
pub async fn load_default_plugins<R: Runtime>(app_handle: AppHandle<R>) -> Result<()> {
    let plugin_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!("Failed to get app data dir: {}", e))?
        .join("plugins");

    if !plugin_dir.exists() {
        // Create the plugins directory if it doesn't exist
        std::fs::create_dir_all(&plugin_dir)
            .map_err(|e| anyhow::anyhow!("Failed to create plugins directory: {}", e))?;
    }

    let manager = PLUGIN_MANAGER
        .get()
        .ok_or_else(|| anyhow::anyhow!("Plugin manager not initialized"))?;

    let mut manager = manager.write().await;

    match manager.load_plugins_from_dir(&plugin_dir) {
        Ok(loaded_plugins) => {
            if !loaded_plugins.is_empty() {
                log::info!("Loaded plugins: {}", loaded_plugins.join(", "));
            } else {
                log::info!("No plugins found in directory: {}", plugin_dir.display());
            }
        },
        Err(e) => {
            log::warn!(
                "Failed to load plugins from directory '{}': {}",
                plugin_dir.display(),
                e
            );
        },
    }

    Ok(())
}

/// Load a specific plugin from a path.
#[command]
#[specta::specta]
pub async fn load_plugin<R: Runtime>(path: String) -> CmdResult<String> {
    let manager = PLUGIN_MANAGER
        .get()
        .ok_or_else(|| anyhow::anyhow!("Plugin manager not initialized"))?;

    let mut manager = manager.write().await;

    manager.load_plugin(&path)?;

    Ok(format!("Plugin loaded successfully from: {}", path))
}

/// Unload a plugin by name.
#[command]
#[specta::specta]
pub async fn unload_plugin<R: Runtime>(name: String) -> CmdResult<String> {
    let manager = PLUGIN_MANAGER
        .get()
        .ok_or_else(|| anyhow::anyhow!("Plugin manager not initialized"))?;

    let mut manager = manager.write().await;

    manager.unload_plugin(&name)?;

    Ok(format!("Plugin '{}' unloaded successfully", name))
}

/// Get information about all loaded plugins.
#[command]
#[specta::specta]
pub async fn list_plugins<R: Runtime>() -> CmdResult<serde_json::Value> {
    let manager = PLUGIN_MANAGER
        .get()
        .ok_or_else(|| anyhow::anyhow!("Plugin manager not initialized"))?;

    let manager = manager.read().await;

    let plugins_info: Vec<_> = manager
        .plugin_info()
        .iter()
        .map(|info| {
            serde_json::json!({
                "name": info.name,
                "version": info.version,
                "commands": info.commands
            })
        })
        .collect();

    Ok(serde_json::json!({
        "plugins": plugins_info,
        "total_plugins": manager.plugin_count(),
        "total_commands": manager.command_count()
    }))
}

/// Call a plugin command.
///
/// # Arguments
/// * `plugin` - The name of the plugin (for backward compatibility, but not
///   used in DLL system)
/// * `command` - The name of the command to call
/// * `id` - The widget ID making the call
/// * `payload` - Optional JSON payload for the command
///
/// # Returns
/// * `Ok(serde_json::Value)` - The result from the plugin command
/// * `Err(CommandError)` - If the command fails
#[command]
#[specta::specta]
pub async fn call_plugin<R: Runtime>(
    _app_handle: AppHandle<R>,
    _plugin: String, // Kept for backward compatibility but not used
    command: String,
    id: String,
    payload: Option<serde_json::Value>,
) -> CmdResult<serde_json::Value> {
    let manager = PLUGIN_MANAGER
        .get()
        .ok_or_else(|| anyhow::anyhow!("Plugin manager not initialized"))?;

    let manager = manager.read().await;

    // Convert payload to JSON string
    let payload_str = match payload {
        Some(value) => serde_json::to_string(&value).map_err(anyhow::Error::from)?,
        None => "null".to_string(),
    };

    // Call the command through the plugin manager
    let result_str = manager.call_command(&command, &id, &payload_str)?;

    // Parse the result back to JSON
    let result: serde_json::Value =
        serde_json::from_str(&result_str).map_err(anyhow::Error::from)?;

    Ok(result)
}

/// Check if a specific command is available.
#[command]
#[specta::specta]
pub async fn has_command<R: Runtime>(command: String) -> CmdResult<bool> {
    let manager = PLUGIN_MANAGER
        .get()
        .ok_or_else(|| anyhow::anyhow!("Plugin manager not initialized"))?;

    let manager = manager.read().await;

    Ok(manager.has_command(&command))
}

/// Get all available command names.
#[command]
#[specta::specta]
pub async fn list_commands<R: Runtime>() -> CmdResult<Vec<String>> {
    let manager = PLUGIN_MANAGER
        .get()
        .ok_or_else(|| anyhow::anyhow!("Plugin manager not initialized"))?;

    let manager = manager.read().await;

    Ok(manager
        .command_names()
        .into_iter()
        .map(|s| s.to_string())
        .collect())
}
