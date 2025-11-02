#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

mod catalog;
mod commands;
mod events;
mod render;
mod setup;

use std::sync::RwLock;

use anyhow::{Result, anyhow, bail};
use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use deskulpt_core::path::PathExt;
use deskulpt_core::states::SettingsStateExt;
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Manager, Runtime, WebviewWindow};

use crate::catalog::WidgetCatalog;
use crate::events::UpdateEvent;
use crate::render::{RenderWorkerHandle, RenderWorkerTask};
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

/// Extension to [`Manager`] for accessing Deskulpt widgets APIs.
trait WidgetsExt<R: Runtime> {
    /// Get a reference to the managed [`Widgets`] state.
    fn widgets(&self) -> &Widgets<R>;
}

impl<R: Runtime, M: Manager<R>> WidgetsExt<R> for M {
    fn widgets(&self) -> &Widgets<R> {
        self.state::<Widgets<R>>().inner()
    }
}

/// Managed state for Deskulpt widgets.
struct Widgets<R: Runtime> {
    /// The Tauri app handle.
    app_handle: AppHandle<R>,
    /// The widget catalog.
    catalog: RwLock<WidgetCatalog>,
    /// The handle for the render worker.
    render_handle: RenderWorkerHandle,
    /// The setup state for frontend windows.
    setup_state: SetupState,
}

impl<R: Runtime> Widgets<R> {
    /// Initialize the [`Widgets`] state.
    fn new(app_handle: AppHandle<R>) -> Self {
        let render_handle = RenderWorkerHandle::new(app_handle.clone());

        Self {
            app_handle,
            catalog: Default::default(),
            render_handle,
            setup_state: Default::default(),
        }
    }

    /// Reload all widgets.
    ///
    /// This method loads a new widget catalog from the widgets directory and
    /// replaces the existing catalog. It then syncs the settings with the
    /// updated catalog. If any step fails, an error is returned.
    fn reload_all(&self) -> Result<()> {
        let widgets_dir = self.app_handle.widgets_dir()?;
        let new_catalog = WidgetCatalog::load(widgets_dir)?;

        let mut catalog = self.catalog.write().unwrap();
        *catalog = new_catalog;
        UpdateEvent(&catalog).emit(&self.app_handle)?;

        self.app_handle
            .update_settings(|settings| catalog.compute_settings_patch(settings))?;
        Ok(())
    }

    /// Render a specific widget by its ID.
    ///
    /// This method submits a render task for the specified widget to the render
    /// worker. If the widget does not exist in the catalog or if task
    /// submission fails, an error is returned. This method is non-blocking and
    /// does not wait for the task to complete.
    fn render(&self, id: String) -> Result<()> {
        let catalog = self.catalog.read().unwrap();
        let config = catalog
            .0
            .get(&id)
            .ok_or_else(|| anyhow!("Widget {id} does not exist in the catalog"))?;

        if let Outcome::Ok(config) = config {
            self.render_handle.process(RenderWorkerTask::Render {
                id,
                entry: config.entry.clone(),
            })?;
        }
        Ok(())
    }

    /// Render all widgets in the catalog.
    ///
    /// This method submits render tasks for all widgets in the catalog to the
    /// render worker. If any task submission fails, an error containing all
    /// accumulated errors is returned. This method is non-blocking and does not
    /// wait for the tasks to complete.
    fn render_all(&self) -> Result<()> {
        let catalog = self.catalog.read().unwrap();

        let mut errors = vec![];
        for (id, config) in catalog.0.iter() {
            if let Outcome::Ok(config) = config
                && let Err(e) = self.render_handle.process(RenderWorkerTask::Render {
                    id: id.clone(),
                    entry: config.entry.clone(),
                })
            {
                errors.push(e.context(format!("Failed to send render task for widget {id}")));
            }
        }

        if !errors.is_empty() {
            let message = errors
                .into_iter()
                .map(|e| format!("{e:?}"))
                .collect::<Vec<_>>()
                .join("\n");
            bail!(message);
        }

        Ok(())
    }

    /// Rescan all widgets.
    ///
    /// This is equivalent to [`Self::reload_all`] then [`Self::render_all`].
    fn rescan(&self) -> Result<()> {
        self.reload_all()?;
        self.render_all()?;
        Ok(())
    }

    /// Bundle widget(s).
    ///
    /// If an ID is provided, the specified widget is bundled with
    /// [`Self::render`]. If no ID is provided, all widgets are bundled with
    /// [`Self::render_all`].
    fn bundle(&self, id: Option<String>) -> Result<()> {
        match id {
            Some(id) => self.render(id)?,
            None => self.render_all()?,
        }
        Ok(())
    }

    /// Mark a window as having completed setup.
    ///
    /// If all windows have completed setup after this call, an initial rescan
    /// is trigger via [`Self::rescan`].
    fn complete_setup(&self, window: WebviewWindow<R>) -> Result<()> {
        let window = window.label().try_into().unwrap();
        let complete = self.setup_state.complete(window);
        if complete {
            self.rescan()?;
        }
        Ok(())
    }
}
