//! Configuration of Deskulpt widgets.

use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use deskulpt_common::outcome::Outcome;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// The Deskulpt widget manifest.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetManifest {
    /// The name of the widget.
    ///
    /// This is purely used for display purposes. It does not need to be related
    /// to the widget directory name, and it does not need to be unique.
    pub name: String,
    /// The entry point of the widget.
    ///
    /// This is the path to the file that exports the widget component. The path
    /// should be relative to the widget directory.
    pub entry: String,
    /// Whether to ignore the widget.
    ///
    /// If set to true, the widget will not be discovered by the application.
    /// This is useful for temporarily disabling a widget without removing it.
    #[serde(default, skip_serializing)]
    ignore: bool,
}

/// The Node.js package manifest.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageManifest {
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

/// Helper trait for loading manifest files from a directory.
trait LoadManifest: Sized + DeserializeOwned {
    /// The name of the manifest file.
    const FILE_NAME: &'static str;

    /// Load the manifest file from the given directory.
    ///
    /// Specially, this method returns `Ok(None)` if the target file does not
    /// exist and does not treat it as an error.
    fn load(dir: &Path) -> Result<Option<Self>> {
        let path = dir.join(Self::FILE_NAME);
        if !path.exists() {
            return Ok(None);
        }
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let manifest = serde_json::from_reader(reader)?;
        Ok(Some(manifest))
    }
}

impl LoadManifest for WidgetManifest {
    const FILE_NAME: &'static str = "deskulpt.widget.json";
}

impl LoadManifest for PackageManifest {
    const FILE_NAME: &'static str = "package.json";
}

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
    /// Read full widget configuration from a directory.
    ///
    /// Specially, this method returns `Ok(None)` if the directory does not
    /// contain a widget configuration file or if the widget is explicitly
    /// marked as ignored in the configuration file.
    pub fn load(dir: &Path) -> Result<Option<Self>> {
        let widget_manifest = match WidgetManifest::load(dir)
            .with_context(|| format!("Failed to load {}", WidgetManifest::FILE_NAME))?
        {
            Some(widget_manifest) if !widget_manifest.ignore => widget_manifest,
            _ => return Ok(None),
        };

        let package_manifest = PackageManifest::load(dir)
            .with_context(|| format!("Failed to load {}", PackageManifest::FILE_NAME))?
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
/// widget IDs to their configurations. Invalid widgets are also included with
/// their error messages.
#[derive(Debug, Default, Clone, Serialize, Deserialize, specta::Type)]
pub struct Catalog(pub BTreeMap<String, Outcome<Config>>);

impl Catalog {
    /// Load the widget catalog from the given directory.
    ///
    /// This scans all top-level subdirectories and attempts to load them as
    /// widgets. Non-widget directories are simply ignored. See
    /// [`WidgetConfig::load`] for more details.
    pub fn load(dir: &Path) -> Result<Self> {
        let mut catalog = Self::default();

        let entries = std::fs::read_dir(dir)?;
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
                catalog.0.insert(id, config);
            }
        }

        Ok(catalog)
    }
}
