//! Plugin registry for managing loaded plugins and their commands.

use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::engine::EngineInterface;
use crate::plugin::Plugin;

/// Registry for managing loaded plugins.
pub struct PluginRegistry {
    /// Map of plugin name to plugin instance
    plugins: HashMap<String, Box<dyn Plugin>>,
    /// Map of command name to (plugin_name, command_index)
    commands: HashMap<String, (String, usize)>,
}

impl PluginRegistry {
    /// Create a new empty plugin registry.
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            commands: HashMap::new(),
        }
    }

    /// Register a plugin in the registry.
    ///
    /// # Arguments
    /// * `name` - The name of the plugin
    /// * `plugin` - The plugin instance
    ///
    /// # Returns
    /// * `Ok(())` - If the plugin was registered successfully
    /// * `Err(anyhow::Error)` - If a plugin with the same name already exists
    pub fn register_plugin(&mut self, name: String, plugin: Box<dyn Plugin>) -> Result<()> {
        if self.plugins.contains_key(&name) {
            return Err(anyhow!("Plugin '{}' is already registered", name));
        }

        // Get commands from the plugin
        let commands = plugin.commands();

        // Register each command
        for (index, command) in commands.iter().enumerate() {
            let command_name = command.name().to_string();
            if self.commands.contains_key(&command_name) {
                return Err(anyhow!(
                    "Command '{}' is already registered by another plugin",
                    command_name
                ));
            }
            self.commands.insert(command_name, (name.clone(), index));
        }

        // Store the plugin
        self.plugins.insert(name, plugin);

        Ok(())
    }

    /// Get a plugin by name.
    ///
    /// # Arguments
    /// * `name` - The name of the plugin
    ///
    /// # Returns
    /// * `Some(&dyn Plugin)` - If the plugin exists
    /// * `None` - If the plugin doesn't exist
    pub fn get_plugin(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|p| p.as_ref())
    }

    /// Call a command by name.
    ///
    /// # Arguments
    /// * `command_name` - The name of the command to call
    /// * `widget_id` - The ID of the widget making the call
    /// * `engine` - The engine interface
    /// * `payload` - The JSON payload as a string
    ///
    /// # Returns
    /// * `Ok(String)` - The JSON result from the command
    /// * `Err(anyhow::Error)` - If the command doesn't exist or fails
    pub fn call_command(
        &self,
        command_name: &str,
        widget_id: &str,
        engine: &EngineInterface,
        payload: &str,
    ) -> Result<String> {
        // Find the command
        let (plugin_name, command_index) = self
            .commands
            .get(command_name)
            .ok_or_else(|| anyhow!("Unknown command: {}", command_name))?;

        // Get the plugin
        let plugin = self
            .plugins
            .get(plugin_name)
            .ok_or_else(|| anyhow!("Plugin '{}' not found", plugin_name))?;

        // Get the command
        let commands = plugin.commands();
        let command = commands
            .get(*command_index)
            .ok_or_else(|| anyhow!("Command index out of bounds"))?;

        // Call the command
        command.run(widget_id, engine, payload)
    }

    /// Get all registered plugin names.
    pub fn plugin_names(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }

    /// Get all registered command names.
    pub fn command_names(&self) -> Vec<&str> {
        self.commands.keys().map(|s| s.as_str()).collect()
    }

    /// Get the number of registered plugins.
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// Get the number of registered commands.
    pub fn command_count(&self) -> usize {
        self.commands.len()
    }

    /// Check if a plugin is registered.
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins.contains_key(name)
    }

    /// Check if a command is registered.
    pub fn has_command(&self, name: &str) -> bool {
        self.commands.contains_key(name)
    }

    /// Clear all registered plugins and commands.
    pub fn clear(&mut self) {
        self.plugins.clear();
        self.commands.clear();
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi::EngineCallbacks;
    use crate::engine::EngineInterface;
    use crate::PluginCommand;

    // Mock plugin for testing
    #[derive(Debug)]
    struct MockPlugin {
        name: String,
        commands: Vec<String>,
    }

    impl MockPlugin {
        fn new(name: &str, commands: Vec<&str>) -> Self {
            Self {
                name: name.to_string(),
                commands: commands.into_iter().map(|s| s.to_string()).collect(),
            }
        }
    }

    impl Plugin for MockPlugin {
        fn name(&self) -> &str {
            &self.name
        }

        fn version(&self) -> String {
            "1.0.0".to_string()
        }

        fn commands(&self) -> Vec<Box<dyn PluginCommand>> {
            self.commands
                .iter()
                .map(|name| Box::new(MockCommand { name: name.clone() }) as Box<dyn PluginCommand>)
                .collect()
        }
    }

    // Mock command for testing
    #[derive(Debug)]
    struct MockCommand {
        name: String,
    }

    impl PluginCommand for MockCommand {
        fn name(&self) -> &str {
            &self.name
        }

        fn run(
            &self,
            _widget_id: &str,
            _engine: &EngineInterface,
            payload: &str,
        ) -> Result<String> {
            // Echo back the command name and payload
            Ok(format!(
                "{{\"command\": \"{}\", \"payload\": {}}}",
                self.name, payload
            ))
        }
    }

    // Mock engine callbacks for testing
    fn create_mock_engine() -> EngineInterface {
        let callbacks = EngineCallbacks {
            widget_dir: mock_widget_dir_callback,
            log: mock_log_callback,
        };
        EngineInterface::new(callbacks)
    }

    unsafe extern "C" fn mock_widget_dir_callback(
        _widget_id: *const std::ffi::c_char,
        _path_out: *mut *mut std::ffi::c_char,
    ) -> i32 {
        // For testing, just return success without setting path
        0
    }

    unsafe extern "C" fn mock_log_callback(_level: i32, _message: *const std::ffi::c_char) {
        // For testing, do nothing
    }

    #[test]
    fn test_new_registry() {
        let registry = PluginRegistry::new();
        assert_eq!(registry.plugin_count(), 0);
        assert_eq!(registry.command_count(), 0);
    }

    #[test]
    fn test_register_plugin() {
        let mut registry = PluginRegistry::new();
        let plugin = Box::new(MockPlugin::new("test-plugin", vec!["cmd1", "cmd2"]));

        let result = registry.register_plugin("test-plugin".to_string(), plugin);
        assert!(result.is_ok());

        assert_eq!(registry.plugin_count(), 1);
        assert_eq!(registry.command_count(), 2);
        assert!(registry.has_plugin("test-plugin"));
        assert!(registry.has_command("cmd1"));
        assert!(registry.has_command("cmd2"));
    }

    #[test]
    fn test_register_duplicate_plugin() {
        let mut registry = PluginRegistry::new();
        let plugin1 = Box::new(MockPlugin::new("test-plugin", vec!["cmd1"]));
        let plugin2 = Box::new(MockPlugin::new("test-plugin", vec!["cmd2"]));

        registry
            .register_plugin("test-plugin".to_string(), plugin1)
            .unwrap();
        let result = registry.register_plugin("test-plugin".to_string(), plugin2);

        assert!(result.is_err());
        assert_eq!(registry.plugin_count(), 1);
        assert_eq!(registry.command_count(), 1);
    }

    #[test]
    fn test_register_duplicate_command() {
        let mut registry = PluginRegistry::new();
        let plugin1 = Box::new(MockPlugin::new("plugin1", vec!["shared-cmd"]));
        let plugin2 = Box::new(MockPlugin::new("plugin2", vec!["shared-cmd"]));

        registry
            .register_plugin("plugin1".to_string(), plugin1)
            .unwrap();
        let result = registry.register_plugin("plugin2".to_string(), plugin2);

        assert!(result.is_err());
        assert_eq!(registry.plugin_count(), 1);
        assert_eq!(registry.command_count(), 1);
        assert!(registry.has_plugin("plugin1"));
        assert!(!registry.has_plugin("plugin2"));
    }

    #[test]
    fn test_get_plugin() {
        let mut registry = PluginRegistry::new();
        let plugin = Box::new(MockPlugin::new("test-plugin", vec!["cmd1"]));

        registry
            .register_plugin("test-plugin".to_string(), plugin)
            .unwrap();

        let retrieved = registry.get_plugin("test-plugin");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name(), "test-plugin");

        let not_found = registry.get_plugin("non-existent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_call_command() {
        let mut registry = PluginRegistry::new();
        let plugin = Box::new(MockPlugin::new("test-plugin", vec!["echo"]));

        registry
            .register_plugin("test-plugin".to_string(), plugin)
            .unwrap();

        let engine = create_mock_engine();
        let result = registry.call_command("echo", "widget123", &engine, "{\"test\": true}");

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("echo"));
        assert!(response.contains("{\"test\": true}"));
    }

    #[test]
    fn test_call_nonexistent_command() {
        let registry = PluginRegistry::new();
        let engine = create_mock_engine();

        let result = registry.call_command("nonexistent", "widget123", &engine, "{}");
        assert!(result.is_err());
    }

    #[test]
    fn test_plugin_names() {
        let mut registry = PluginRegistry::new();
        let plugin1 = Box::new(MockPlugin::new("plugin1", vec!["cmd1"]));
        let plugin2 = Box::new(MockPlugin::new("plugin2", vec!["cmd2"]));

        registry
            .register_plugin("plugin1".to_string(), plugin1)
            .unwrap();
        registry
            .register_plugin("plugin2".to_string(), plugin2)
            .unwrap();

        let mut names = registry.plugin_names();
        names.sort();
        assert_eq!(names, vec!["plugin1", "plugin2"]);
    }

    #[test]
    fn test_command_names() {
        let mut registry = PluginRegistry::new();
        let plugin1 = Box::new(MockPlugin::new("plugin1", vec!["cmd1", "cmd2"]));
        let plugin2 = Box::new(MockPlugin::new("plugin2", vec!["cmd3"]));

        registry
            .register_plugin("plugin1".to_string(), plugin1)
            .unwrap();
        registry
            .register_plugin("plugin2".to_string(), plugin2)
            .unwrap();

        let mut commands = registry.command_names();
        commands.sort();
        assert_eq!(commands, vec!["cmd1", "cmd2", "cmd3"]);
    }

    #[test]
    fn test_clear() {
        let mut registry = PluginRegistry::new();
        let plugin = Box::new(MockPlugin::new("test-plugin", vec!["cmd1"]));

        registry
            .register_plugin("test-plugin".to_string(), plugin)
            .unwrap();
        assert_eq!(registry.plugin_count(), 1);
        assert_eq!(registry.command_count(), 1);

        registry.clear();
        assert_eq!(registry.plugin_count(), 0);
        assert_eq!(registry.command_count(), 0);
        assert!(!registry.has_plugin("test-plugin"));
        assert!(!registry.has_command("cmd1"));
    }

    #[test]
    fn test_multiple_commands_per_plugin() {
        let mut registry = PluginRegistry::new();
        let plugin = Box::new(MockPlugin::new(
            "multi-cmd-plugin",
            vec!["cmd1", "cmd2", "cmd3"],
        ));

        registry
            .register_plugin("multi-cmd-plugin".to_string(), plugin)
            .unwrap();

        assert_eq!(registry.plugin_count(), 1);
        assert_eq!(registry.command_count(), 3);

        let engine = create_mock_engine();

        for cmd in ["cmd1", "cmd2", "cmd3"] {
            let result = registry.call_command(cmd, "widget123", &engine, "{}");
            assert!(result.is_ok());
            assert!(result.unwrap().contains(cmd));
        }
    }

    #[test]
    fn test_empty_plugin() {
        let mut registry = PluginRegistry::new();
        let plugin = Box::new(MockPlugin::new("empty-plugin", vec![]));

        let result = registry.register_plugin("empty-plugin".to_string(), plugin);
        assert!(result.is_ok());

        assert_eq!(registry.plugin_count(), 1);
        assert_eq!(registry.command_count(), 0);
        assert!(registry.has_plugin("empty-plugin"));
    }
}
