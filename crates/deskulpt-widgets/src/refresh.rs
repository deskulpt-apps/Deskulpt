use anyhow::{Result, bail};
use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use tauri::Runtime;

use crate::WidgetsManager;
use crate::events::UpdateEvent;
use crate::model::{Widget, Widgets};
use crate::render::RenderWorkerTask;

impl<R: Runtime> WidgetsManager<R> {
    /// Reload a specific widget by its ID.
    ///
    /// This method loads the widget manifest from the corresponding widget
    /// directory and updates the catalog entry for that widget. This could be
    /// an addition, removal, or modification. It then syncs the settings with
    /// the updated catalog. If any step fails, an error is returned.
    pub fn reload(&self, id: &str) -> Result<bool> {
        let widget_dir = self.dir.join(id);
        let widget = Widget::load(&widget_dir);

        let mut widgets = self.widgets.write();
        let exists = widgets.insert_or_remove(id.to_string(), widget);

        UpdateEvent(&widgets).emit(&self.app_handle)?;
        Ok(exists)
    }

    /// Reload all widgets.
    ///
    /// This method loads a new widget catalog from the widgets directory and
    /// replaces the existing catalog. It then syncs the settings with the
    /// updated catalog. If any step fails, an error is returned.
    pub fn reload_all(&self) -> Result<()> {
        let new_widgets = Widgets::load(&self.dir)?;

        let mut widgets = self.widgets.write();
        *widgets = new_widgets;

        UpdateEvent(&widgets).emit(&self.app_handle)?;
        Ok(())
    }

    /// Render a specific widget by its ID.
    ///
    /// This method submits a render task for the specified widget to the render
    /// worker. If the widget does not exist in the catalog or if task
    /// submission fails, an error is returned. This method is non-blocking and
    /// does not wait for the task to complete.
    pub(crate) fn render(&self, id: &str) -> Result<()> {
        let widgets = self.widgets.read();
        let widget = widgets.get(id)?;

        if let Outcome::Ok(manifest) = &widget.manifest {
            self.render_worker.process(RenderWorkerTask::Render {
                id: id.to_string(),
                entry: manifest.entry.clone(),
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
    pub(crate) fn render_all(&self) -> Result<()> {
        let widgets = self.widgets.read();

        let mut errors = vec![];
        for (id, widget) in widgets.iter() {
            if let Outcome::Ok(manifest) = &widget.manifest
                && let Err(e) = self.render_worker.process(RenderWorkerTask::Render {
                    id: id.clone(),
                    entry: manifest.entry.clone(),
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

    /// Refresh a specific widget by its ID.
    ///
    /// This is equivalent to reloading that widget with [`Self::reload`] then
    /// rendering it with [`Self::render`].
    ///
    /// Tauri command: [`crate::commands::refresh`].
    pub fn refresh(&self, id: &str) -> Result<()> {
        self.reload(id)?;
        self.render(id)?;
        Ok(())
    }

    /// Refresh all widgets.
    ///
    /// This is equivalent to reloading all widgets with [`Self::reload_all`]
    /// then rendering all widgets with [`Self::render_all`].
    ///
    /// Tauri command: [`crate::commands::refresh_all`].
    pub(crate) fn refresh_all(&self) -> Result<()> {
        self.reload_all()?;
        self.render_all()?;
        Ok(())
    }
}
