//! Plugin command APIs.

use anyhow::Result;

use crate::Plugin;
use crate::interface::EngineInterface;

/// The API for a Deskulpt plugin command.
pub trait PluginCommand {
    /// The type of the plugin the command runs on.
    ///
    /// This is needed for the [`run`](PluginCommand::run) method to take a
    /// reference back to the plugin.
    type Plugin: Plugin;

    /// The name of the command.
    fn name(&self) -> &str;

    /// The implementation of the command.
    ///
    /// One should almost always use the [`#[dispatch]`](macro@crate::dispatch)
    /// attribute when implementing this method, which allows specifying custom
    /// types for the `input` argument and the return value and automatically
    /// handles their gaps with the expected signature of this method. See
    /// [`#[dispatch]`](macro@crate::dispatch) documentation for details and
    /// examples.
    ///
    /// Other available information include:
    ///
    /// - `id` is the ID of the widget that triggered the command.
    /// - `plugin` provides a reference back to the plugin that the command is
    ///   running on. This is useful when the plugin carries some state that the
    ///   command needs to access.
    /// - `engine` provides an interface for interacting with the Deskulpt
    ///   engine. See [`EngineInterface`] for available methods.
    fn run(
        &self,
        id: String,
        plugin: &Self::Plugin,
        engine: &EngineInterface,
        input: serde_json::Value,
    ) -> Result<serde_json::Value>;
}
