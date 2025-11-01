#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

mod bundler;
mod catalog;
mod commands;
mod events;
mod setup;

use std::sync::RwLock;

use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Manager, Runtime};

use crate::bundler::RenderWorkerHandle;
use crate::catalog::Catalog;
use crate::setup::SetupState;

deskulpt_common::bindings::build_bindings!();

/// Initialize the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    deskulpt_common::init::init_builder!()
        .setup(|app_handle, _| {
            app_handle.manage(Widgets::new(app_handle.clone()));
            app_handle.manage(SetupState::default());
            Ok(())
        })
        .build()
}

/// Managed state for Deskulpt widgets.
pub struct Widgets<R: Runtime> {
    /// An app handle embedded for convenience.
    app_handle: AppHandle<R>,
    /// The widget catalog.
    catalog: RwLock<Catalog>,
    /// The handle for the render worker.
    render_handle: RenderWorkerHandle,
}

impl<R: Runtime> Widgets<R> {
    /// Initialize the [`Widgets`] state.
    pub fn new(app_handle: AppHandle<R>) -> Self {
        let render_handle = RenderWorkerHandle::new(app_handle.clone());
        Self {
            app_handle,
            catalog: Default::default(),
            render_handle,
        }
    }
}

/// Extension to [`Manager`] for accessing Deskulpt widgets APIs.
pub trait WidgetsExt<R: Runtime> {
    /// Get a reference to the managed [`Widgets`] state.
    fn widgets(&self) -> &Widgets<R>;
}

impl<R: Runtime, M: Manager<R>> WidgetsExt<R> for M {
    fn widgets(&self) -> &Widgets<R> {
        self.state::<Widgets<R>>().inner()
    }
}
