#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

mod command;
mod interface;

use std::path::PathBuf;

use anyhow::{bail, Result};
pub use command::PluginCommand;
pub use interface::EngineInterface;
pub use {anyhow, serde_json};

/// The API for a Deskulpt plugin.
pub trait Plugin {
    /// The version of the plugin.
    ///
    /// The default (recommended) implementation uses the version as specified
    /// in `Cargo.toml` for the plugin.
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// The commands provided by the plugin.
    ///
    /// One may use the [`register_commands!`] macro for a convenient way to
    /// implement this method.
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>>;
}

/// Call a Deskulpt plugin (🚧 TODO 🚧).
///
/// ### 🚧 TODO 🚧
///
/// This function should be completed removed and replaced with a `serve_plugin`
/// function that will serve as the entry point of the plugin, running it as a
/// standalone process that can interact with the Deskulpt core through IPC. See
/// [nushell](https://docs.rs/nu-plugin/0.101.0/nu_plugin/fn.serve_plugin.html)
/// for reference.
pub fn call_plugin<P: Plugin>(
    widget_dir_fn: impl Fn(&str) -> Result<PathBuf> + 'static,
    plugin: &P,
    command: &str,
    id: String,
    payload: Option<serde_json::Value>,
) -> Result<serde_json::Value> {
    let engine = EngineInterface::new(widget_dir_fn);

    for plugin_command in plugin.commands() {
        if plugin_command.name() == command {
            return plugin_command.run(
                id,
                plugin,
                &engine,
                payload.unwrap_or(serde_json::Value::Null),
            );
        }
    }
    bail!("Unknown command: {}", command)
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
        fn commands(&self) -> Vec<Box<dyn $crate::PluginCommand<Plugin = Self>>> {
            vec![$(Box::new($command),)*]
        }
    };
}

/// Dispatch a Deskulpt plugin command.
///
/// The [`PluginCommand::run`] method requires the [`serde_json::Value`] type
/// for command input and output so as to interoperate with calls from the
/// widgets in the frontend. This would require manual deserialization and
/// serialization when implementing any command.
///
/// When marked with `#[dispatch]`, the signature of the method remains the
/// same, except that `input` is allowed to be any type that implements
/// [`serde::Deserialize`], and the return type is allowed to be `Result<T, E>`
/// for any type `T` that implements [`serde::Serialize`] and any type `E` that
/// can be converted to [`anyhow::Error`] with the `?` operator. That said, the
/// most convenient way would be to use [`anyhow::Result<T>`](anyhow::Result)
/// for the return type directly.
///
/// ### Example
///
/// ```no_run
/// use anyhow::Result;
/// use deskulpt_plugin::{dispatch, EngineInterface, PluginCommand};
/// # use deskulpt_plugin::{register_commands, Plugin};
/// use serde::{Deserialize, Serialize};
///
/// // Implement the plugin...
/// # struct MyPlugin;
/// #
/// # impl Plugin for MyPlugin {
/// #     register_commands![MetadataCommand];
/// # }
///
/// struct MetadataCommand;
///
/// #[derive(Deserialize)]
/// struct InputPayload {
///     path: std::path::PathBuf,
/// }
///
/// #[derive(Serialize)]
/// struct OutputPayload {
///     is_dir: bool,
///     is_file: bool,
///     is_symlink: bool,
///     len: u64,
/// }
///
/// impl PluginCommand for MetadataCommand {
///     // Associate types and methods...
///     # type Plugin = MyPlugin;
///     #
///     # fn name(&self) -> &str {
///     #     "metadata"
///     # }
///
///     #[dispatch]
///     fn run(
///         &self,
///         _id: String,
///         _plugin: &Self::Plugin,
///         _engine: &EngineInterface,
///         input: InputPayload,     // Custom deserializable input type
///     ) -> Result<OutputPayload> { // Custom serializable output type
///         let metadata = std::fs::metadata(input.path)?;
///         Ok(OutputPayload {
///             is_dir: metadata.is_dir(),
///             is_file: metadata.is_file(),
///             is_symlink: metadata.file_type().is_symlink(),
///             len: metadata.len(),
///         })
///     }
/// }
/// ```
pub use deskulpt_plugin_macros::dispatch;
