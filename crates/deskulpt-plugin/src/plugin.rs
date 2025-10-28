//! Plugin and command trait definitions.

use anyhow::Result;

use crate::engine::EngineInterface;

/// Information about a plugin.
#[derive(Debug, Clone)]
pub struct PluginInfo {
    /// The name of the plugin
    pub name: String,
    /// The version of the plugin
    pub version: String,
    /// List of available command names
    pub commands: Vec<String>,
}

/// The API for a Deskulpt plugin.
pub trait Plugin: Send + Sync {
    /// The name of the plugin.
    fn name(&self) -> &str;

    /// The version of the plugin.
    ///
    /// The default implementation uses the version as specified
    /// in `Cargo.toml` for the plugin.
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// The commands provided by the plugin.
    ///
    /// One may use the [`register_commands!`] macro for a convenient way to
    /// implement this method.
    fn commands(&self) -> Vec<Box<dyn PluginCommand>>;
}

/// The API for a Deskulpt plugin command.
pub trait PluginCommand: Send + Sync {
    /// The name of the command.
    fn name(&self) -> &str;

    /// The implementation of the command.
    ///
    /// # Arguments
    /// * `widget_id` - The ID of the widget that triggered the command
    /// * `engine` - Interface for interacting with the Deskulpt engine
    /// * `payload` - JSON payload as a string
    ///
    /// # Returns
    /// * `Ok(String)` - JSON result as a string
    /// * `Err(anyhow::Error)` - If the command failed
    fn run(&self, widget_id: &str, engine: &EngineInterface, payload: &str) -> Result<String>;
}

/// A convenience trait for implementing plugin commands with typed
/// input/output.
///
/// This trait provides automatic JSON serialization/deserialization for command
/// input and output, making it easier to work with strongly typed data.
pub trait TypedPluginCommand: Send + Sync {
    /// The input type for the command (must implement Deserialize).
    type Input: serde::de::DeserializeOwned;

    /// The output type for the command (must implement Serialize).
    type Output: serde::Serialize;

    /// The name of the command.
    fn name(&self) -> &str;

    /// The typed implementation of the command.
    ///
    /// # Arguments
    /// * `widget_id` - The ID of the widget that triggered the command
    /// * `engine` - Interface for interacting with the Deskulpt engine
    /// * `input` - Deserialized input data
    ///
    /// # Returns
    /// * `Ok(Self::Output)` - The command result
    /// * `Err(anyhow::Error)` - If the command failed
    fn run_typed(
        &self,
        widget_id: &str,
        engine: &EngineInterface,
        input: Self::Input,
    ) -> Result<Self::Output>;
}

/// Automatic implementation of PluginCommand for any TypedPluginCommand.
impl<T: TypedPluginCommand> PluginCommand for T {
    fn name(&self) -> &str {
        TypedPluginCommand::name(self)
    }

    fn run(&self, widget_id: &str, engine: &EngineInterface, payload: &str) -> Result<String> {
        // Deserialize input
        let input: T::Input = if payload.trim().is_empty() {
            // Handle empty payload case - try to deserialize from null
            serde_json::from_str("null")?
        } else {
            serde_json::from_str(payload)?
        };

        // Call typed implementation
        let output = self.run_typed(widget_id, engine, input)?;

        // Serialize output
        let result = serde_json::to_string(&output)?;
        Ok(result)
    }
}
