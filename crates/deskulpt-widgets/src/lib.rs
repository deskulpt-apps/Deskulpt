#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

mod commands;
mod config;
mod manifests;
mod rescan;

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Mutex;

use deskulpt_common::outcome::Outcome;
use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

use crate::config::Config;

deskulpt_common::bindings::configure_bindings_builder!();

struct WidgetsInner {
    dir: PathBuf,
    catalog: BTreeMap<String, Outcome<Config>>,
}

/// Managed state for Deskulpt widgets.
pub struct Widgets(Mutex<WidgetsInner>);

pub trait WidgetsExt<R: Runtime> {
    fn widgets(&self) -> &Widgets;
}

impl<R: Runtime, M: Manager<R>> WidgetsExt<R> for M {
    fn widgets(&self) -> &Widgets {
        self.state::<Widgets>().inner()
    }
}

/// Initialize the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    deskulpt_common::init::init_builder!()
        .setup(|app_handle, _| {
            // TODO(Charlie-XIAO): placeholder
            app_handle.manage(Widgets(Mutex::new(WidgetsInner {
                dir: PathBuf::new(),
                catalog: BTreeMap::new(),
            })));
            Ok(())
        })
        .build()
}
