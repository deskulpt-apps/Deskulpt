//! Configuration of Deskulpt widgets.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use anyhow::{Context, Result};
use deskulpt_common::outcome::Outcome;
use serde::{Deserialize, Serialize};

use crate::manifests::{LoadManifest, PackageManifest, WidgetManifest};

/// Full configuration of a Deskulpt widget.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// The name of the widget.
    pub name: String,
    /// The entry point of the widget.
    pub entry: String,
    /// The dependencies of the widget.
    pub dependencies: HashMap<String, String>,
}

impl Config {
    /// Read widget configuration from a directory.
    ///
    /// Specially, this method returns `Ok(None)` if the directory does not
    /// contain a widget configuration file or if the widget is explicitly
    /// marked as ignored in the configuration file.
    pub fn load(dir: &Path) -> Result<Option<Self>> {
        let widget_manifest =
            match WidgetManifest::load(dir).context("Failed to load deskulpt.conf.json")? {
                Some(widget_manifest) if !widget_manifest.ignore => widget_manifest,
                _ => return Ok(None),
            };

        let package_manifest = PackageManifest::load(dir)
            .context("Failed to load package.json")?
            .unwrap_or_default();

        Ok(Some(Config {
            name: widget_manifest.name,
            entry: widget_manifest.entry,
            dependencies: package_manifest.dependencies,
        }))
    }
}

/// The widget catalog.
///
/// This is a collection of all widgets discovered locally, mapped from their
/// widget IDs to their configurations.
#[derive(Debug, Default, Clone, Serialize, Deserialize, specta::Type)]
pub struct Catalog(pub BTreeMap<String, Outcome<Config>>);
