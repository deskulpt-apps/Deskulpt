//! Plugin manager for managing multiple loaded plugins.

use std::collections::HashMap;
use std::path::Path;

use anyhow::{anyhow, Result};

use super::{EngineCallbacks, LoadedPlugin, PluginLoader};

/// Manager for multiple loaded plugins.
pub struct PluginManager {
    /// Map of plugin name to loaded plugin
    plugins: HashMap<String, LoadedPlugin>,
    /// Engine callbacks provided to plugins
    engine_callbacks: EngineCallbacks,
}

impl PluginManager {
    /// Create a new plugin manager with the given engine callbacks.
    pub fn new(engine_callbacks: EngineCallbacks) -> Self {
        Self {
            plugins: HashMap::new(),
            engine_callbacks,
        }
    }

    /// Load a plugin from the specified path.
    ///
    /// # Arguments
    /// * `path` - Path to the plugin DLL
    ///
    /// # Returns
    /// * `Ok(())` - If the plugin was loaded successfully
    /// * `Err(anyhow::Error)` - If the plugin failed to load or conflicts with
    ///   existing plugins
    pub fn load_plugin<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let plugin = PluginLoader::load_plugin(path, self.engine_callbacks)?;
        let plugin_name = plugin.info().name.clone();

        // Check for name conflicts
        if self.plugins.contains_key(&plugin_name) {
            return Err(anyhow!("Plugin '{}' is already loaded", plugin_name));
        }

        // Check for command conflicts
        for command_name in &plugin.info().commands {
            if self.has_command(command_name) {
                return Err(anyhow!(
                    "Command '{}' from plugin '{}' conflicts with already loaded plugin",
                    command_name,
                    plugin_name
                ));
            }
        }

        self.plugins.insert(plugin_name, plugin);
        Ok(())
    }

    /// Load all plugins from a directory.
    ///
    /// # Arguments
    /// * `dir` - Directory containing plugin DLLs
    ///
    /// # Returns
    /// * `Ok(Vec<String>)` - Names of successfully loaded plugins
    /// * `Err(anyhow::Error)` - If the directory cannot be read
    pub fn load_plugins_from_dir<P: AsRef<Path>>(&mut self, dir: P) -> Result<Vec<String>> {
        let dir = dir.as_ref();

        if !dir.exists() || !dir.is_dir() {
            return Err(anyhow!(
                "Plugin directory does not exist: {}",
                dir.display()
            ));
        }

        let mut loaded_plugins = Vec::new();
        let mut errors = Vec::new();

        let entries = std::fs::read_dir(dir)
            .map_err(|e| anyhow!("Failed to read plugin directory '{}': {}", dir.display(), e))?;

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    errors.push(format!("Failed to read directory entry: {}", e));
                    continue;
                },
            };

            let path = entry.path();

            if !PluginLoader::is_valid_plugin_path(&path) {
                continue;
            }

            match self.load_plugin(&path) {
                Ok(()) => {
                    // Get the plugin name that was just loaded
                    if let Some(plugin_name) = self.plugins.keys().last() {
                        loaded_plugins.push(plugin_name.clone());
                    }
                },
                Err(e) => {
                    errors.push(format!("Failed to load plugin '{}': {}", path.display(), e));
                },
            }
        }

        if !errors.is_empty() {
            log::warn!("Some plugins failed to load:\n{}", errors.join("\n"));
        }

        Ok(loaded_plugins)
    }

    /// Unload a plugin by name.
    ///
    /// # Arguments
    /// * `name` - Name of the plugin to unload
    ///
    /// # Returns
    /// * `Ok(())` - If the plugin was unloaded successfully
    /// * `Err(anyhow::Error)` - If the plugin doesn't exist
    pub fn unload_plugin(&mut self, name: &str) -> Result<()> {
        if self.plugins.remove(name).is_some() {
            Ok(())
        } else {
            Err(anyhow!("Plugin '{}' is not loaded", name))
        }
    }

    /// Call a command from any loaded plugin.
    ///
    /// # Arguments
    /// * `command_name` - Name of the command to call
    /// * `widget_id` - ID of the widget making the call
    /// * `payload` - JSON payload as a string
    ///
    /// # Returns
    /// * `Ok(String)` - JSON result from the command
    /// * `Err(anyhow::Error)` - If the command doesn't exist or fails
    pub fn call_command(
        &self,
        command_name: &str,
        widget_id: &str,
        payload: &str,
    ) -> Result<String> {
        // Find the plugin that has this command
        for plugin in self.plugins.values() {
            if plugin.has_command(command_name) {
                return plugin.call_command(command_name, widget_id, payload);
            }
        }

        Err(anyhow!("Unknown command: {}", command_name))
    }

    /// Get a plugin by name.
    pub fn get_plugin(&self, name: &str) -> Option<&LoadedPlugin> {
        self.plugins.get(name)
    }

    /// Check if a plugin is loaded.
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins.contains_key(name)
    }

    /// Check if a command is available.
    pub fn has_command(&self, command_name: &str) -> bool {
        self.plugins
            .values()
            .any(|plugin| plugin.has_command(command_name))
    }

    /// Get all loaded plugin names.
    pub fn plugin_names(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }

    /// Get all available command names.
    pub fn command_names(&self) -> Vec<&str> {
        self.plugins
            .values()
            .flat_map(|plugin| plugin.command_names())
            .map(|s| s.as_str())
            .collect()
    }

    /// Get the number of loaded plugins.
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// Get the total number of available commands.
    pub fn command_count(&self) -> usize {
        self.plugins
            .values()
            .map(|plugin| plugin.command_names().len())
            .sum()
    }

    /// Get plugin information for all loaded plugins.
    pub fn plugin_info(&self) -> Vec<&super::PluginInfo> {
        self.plugins.values().map(|plugin| plugin.info()).collect()
    }

    /// Unload all plugins.
    pub fn unload_all(&mut self) {
        self.plugins.clear();
    }

    /// Get plugin that provides a specific command.
    pub fn find_plugin_for_command(&self, command_name: &str) -> Option<&LoadedPlugin> {
        self.plugins
            .values()
            .find(|plugin| plugin.has_command(command_name))
    }

    /// Reload a plugin by name.
    ///
    /// This will unload the existing plugin and load it again from the same
    /// path. Note: This requires storing the original path, which is not
    /// currently implemented.
    pub fn reload_plugin(&mut self, _name: &str) -> Result<()> {
        // TODO: Implement plugin reloading
        // This would require storing the original path of each plugin
        Err(anyhow!("Plugin reloading not yet implemented"))
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        // Plugins will be automatically destroyed when the HashMap is dropped
        // due to the Drop implementation of LoadedPlugin
    }
}
