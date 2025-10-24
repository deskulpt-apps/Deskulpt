use std::collections::BTreeMap;

use anyhow::Result;
use deskulpt_common::outcome::Outcome;

use crate::config::Config;
use crate::Widgets;

impl Widgets {
    pub fn rescan(&self) -> Result<()> {
        let inner = self.0.lock().unwrap();
        let mut new_catalog = BTreeMap::new();

        let entries = std::fs::read_dir(&inner.dir)?;
        for entry in entries {
            let entry = entry?;

            let path = entry.path();
            if !path.is_dir() {
                continue; // Non-directory entries are not widgets, skip
            }

            if let Some(config) = Config::load(&path)
                .map(|opt| opt.map(Outcome::Ok))
                .unwrap_or_else(|e| Some(Outcome::Err(format!("{e:?}"))))
            {
                // Since each widget must be at the top level of the widgets
                // directory, the directory names must be unique and we can use
                // them as widget IDs
                let id = entry.file_name().to_string_lossy().to_string();
                new_catalog.insert(id, config);
            }
        }

        let mut inner = inner;
        inner.catalog = new_catalog;
        Ok(())
    }
}
