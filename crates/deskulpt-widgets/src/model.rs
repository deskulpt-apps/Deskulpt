use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Result, anyhow};
use deskulpt_common::outcome::Outcome;
use deskulpt_manifest::{LoadManifest, WidgetManifest};
use serde::{Deserialize, Serialize};

use crate::settings::WidgetSettings;

#[derive(Clone, Debug, Deserialize, Serialize, specta::Type)]
pub(crate) struct Widget {
    pub(crate) manifest: Outcome<WidgetManifest>,
    pub(crate) settings: WidgetSettings,
}

impl Widget {
    pub(crate) fn load(dir: &Path) -> Option<Self> {
        let manifest = WidgetManifest::load(dir).transpose()?;
        let settings = match &manifest {
            Ok(m) => WidgetSettings::from_manifest(m),
            Err(_) => WidgetSettings::default(),
        };

        Some(Self {
            manifest: manifest.into(),
            settings,
        })
    }

    pub(crate) fn covers_point(&self, x: f64, y: f64) -> bool {
        let sx = self.settings.x as f64;
        let sy = self.settings.y as f64;
        let ex = sx + self.settings.width as f64;
        let ey = sy + self.settings.height as f64;

        x >= sx && x <= ex && y >= sy && y <= ey
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, specta::Type)]
pub(crate) struct Widgets(BTreeMap<String, Widget>);

impl Widgets {
    pub(crate) fn get(&self, id: &str) -> Result<&Widget> {
        self.0
            .get(id)
            .ok_or_else(|| anyhow!("Widget not found: {id}"))
    }

    pub(crate) fn get_mut(&mut self, id: &str) -> Result<&mut Widget> {
        self.0
            .get_mut(id)
            .ok_or_else(|| anyhow!("Widget not found: {id}"))
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&String, &Widget)> {
        self.0.iter()
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = &Widget> {
        self.0.values()
    }

    pub(crate) fn insert_or_remove(&mut self, id: String, widget: Option<Widget>) -> bool {
        match widget {
            Some(w) => {
                self.0.insert(id, w);
                true
            },
            None => {
                self.0.remove(&id);
                false
            },
        }
    }

    pub(crate) fn load(dir: &Path) -> Result<Self> {
        let mut widgets = Self::default();

        let entries = std::fs::read_dir(dir)?;
        for entry in entries {
            let entry = entry?;

            let path = entry.path();
            if !path.is_dir() {
                continue; // Non-directory entries are not widgets, skip
            }

            if let Some(widget) = Widget::load(&path) {
                // Since each widget must be at the top level of the widgets
                // directory, the directory names must be unique and we can use
                // them as widget IDs
                let id = entry.file_name().to_string_lossy().to_string();
                widgets.0.insert(id, widget);
            }
        }

        Ok(widgets)
    }

    // pub(crate) fn insert(&mut self, id: String, widget: Widget) -> Option<Widget>
    // {     self.0.insert(id, widget)
    // }

    // pub(crate) fn remove(&mut self, id: &str) -> Option<Widget> {
    //     self.0.remove(id)
    // }
}
