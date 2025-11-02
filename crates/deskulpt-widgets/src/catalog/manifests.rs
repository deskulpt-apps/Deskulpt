//! Manifest file definitions and loading utilities.

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use serde::Deserialize;
use serde::de::DeserializeOwned;

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
    pub ignore: bool,
}

/// The Node.js package manifest.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageManifest {
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

/// Helper trait for loading manifest files from a directory.
pub trait LoadManifest: Sized + DeserializeOwned {
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
        let config = serde_json::from_reader(reader)?;
        Ok(Some(config))
    }
}

impl LoadManifest for WidgetManifest {
    const FILE_NAME: &'static str = "deskulpt.widget.json";
}

impl LoadManifest for PackageManifest {
    const FILE_NAME: &'static str = "package.json";
}
