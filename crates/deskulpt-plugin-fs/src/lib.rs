#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

mod commands;

use deskulpt_plugin::{implement_plugin, register_commands, Plugin};

/// The file system plugin.
///
/// This plugin provides file system operations for widgets, including
/// reading, writing, and manipulating files and directories within
/// the widget's designated directory.
#[derive(Default)]
pub struct FsPlugin;

impl Plugin for FsPlugin {
    fn name(&self) -> &str {
        "fs"
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    register_commands![
        commands::AppendFile,
        commands::CreateDir,
        commands::Exists,
        commands::IsDir,
        commands::IsFile,
        commands::ReadFile,
        commands::RemoveDir,
        commands::RemoveFile,
        commands::WriteFile,
    ];
}

// Implement the required C ABI exports for the plugin
implement_plugin!(FsPlugin);
