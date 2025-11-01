#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

mod commands;

use std::sync::Mutex;

use deskulpt_plugin::{Plugin, register_commands};
use sysinfo::System;

/// The system information plugin (🚧 TODO 🚧).
///
/// ### 🚧 TODO 🚧
///
/// Redesign the exposed APIs, splitting into different groups of information to
/// avoid having to retrieve all information even when only a subset is needed.
///
/// Also note that the `#[derive(Default)]` may be removed if unneeded.
#[derive(Default)]
pub struct SysPlugin(pub Mutex<System>);

impl Plugin for SysPlugin {
    register_commands![commands::GetSystemInfo];
}
