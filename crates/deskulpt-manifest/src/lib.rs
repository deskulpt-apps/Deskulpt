#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Author information in a manifest.
#[derive(Clone, Debug, Deserialize, Serialize, specta::Type)]
#[serde(untagged)]
pub enum ManifestAuthor {
    /// The name of the author.
    ///
    /// If a string is given, it will be deserialized into this variant.
    Name(String),
    /// An extended author with name, email, and homepage.
    ///
    /// If an object is given, it will be deserialized into this variant.
    #[serde(rename_all = "camelCase")]
    Extended {
        /// The name of the author.
        name: String,
        /// An optional email of the author.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[specta(type = String)]
        email: Option<String>,
        /// An optional URL to the homepage of the author.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[specta(type = String)]
        url: Option<String>,
    },
}

impl ManifestAuthor {
    /// Get the name of the author.
    pub fn name(&self) -> &str {
        match self {
            ManifestAuthor::Name(name) => name,
            ManifestAuthor::Extended { name, .. } => name,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ManifestMetadata {
    /// The display name of the item.
    pub name: String,
    /// The version of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = String)]
    pub version: Option<String>,
    /// The authors of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = Vec<ManifestAuthor>)]
    pub authors: Option<Vec<ManifestAuthor>>,
    /// The license of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = String)]
    pub license: Option<String>,
    /// A short description of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = String)]
    pub description: Option<String>,
    /// URL to the homepage of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = String)]
    pub homepage: Option<String>,
    /// Whether to ignore the item.
    ///
    /// If set to true, the item will not be discovered by the application,
    /// despite the presence of the manifest file.
    #[serde(default, skip_serializing)]
    pub ignore: bool,
}

/// Deskulpt widget manifest.
#[derive(Clone, Debug, Default, Deserialize, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct WidgetManifest {
    /// The entry module of the widget that exports the widget component.
    ///
    /// This is a path relative to the root of the widget.
    pub entry: String,
    /// The metadata of the widget.
    #[serde(flatten)]
    pub metadata: ManifestMetadata,
}

/// Deskulpt plugin manifest.
#[derive(Clone, Debug, Default, Deserialize, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {
    /// The metadata of the plugin.
    #[serde(flatten)]
    pub metadata: ManifestMetadata,
}

pub trait LoadManifest: Sized + DeserializeOwned {
    /// The name of the manifest file.
    const FILE_NAME: &'static str;

    /// Whether the manifest explicitly marks itself as ignored.
    fn ignored(&self) -> bool;

    /// Load the manifest from a directory.
    ///
    /// This method returns `Ok(None)` if the directory does not contain a file
    /// named [`Self::FILE_NAME`], or if the manifest explicitly marks itself as
    /// ignored. If loading or parsing the manifest fails, an error is returned.
    /// Otherwise, the manifest is returned wrapped in `Ok(Some(...))`.
    fn load(dir: &Path) -> Result<Option<Self>> {
        let path = dir.join(Self::FILE_NAME);
        if !path.exists() {
            return Ok(None);
        }

        let file = File::open(&path)
            .with_context(|| format!("Failed to open widget manifest: {}", path.display()))?;
        let reader = BufReader::new(file);
        let manifest: Self = serde_json::from_reader(reader)
            .with_context(|| format!("Failed to parse widget manifest: {}", path.display()))?;

        Ok(if manifest.ignored() {
            None
        } else {
            Some(manifest)
        })
    }
}

impl LoadManifest for WidgetManifest {
    const FILE_NAME: &str = "deskulpt.widget.json";

    fn ignored(&self) -> bool {
        self.metadata.ignore
    }
}

impl LoadManifest for PluginManifest {
    const FILE_NAME: &str = "deskulpt.plugin.json";

    fn ignored(&self) -> bool {
        self.metadata.ignore
    }
}
