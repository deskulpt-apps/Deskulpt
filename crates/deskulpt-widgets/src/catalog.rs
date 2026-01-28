//! Deskulpt widget manifest and catalog.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use deskulpt_common::outcome::Outcome;
use serde::{Deserialize, Serialize};

/// An author of a Deskulpt widget.
#[derive(Clone, Debug, Deserialize, Serialize, specta::Type)]
#[serde(untagged)]
pub enum ManifestAuthor {
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
        homepage: Option<String>,
    },
    /// The name of the author.
    ///
    /// If a string is given, it will be deserialized into this variant.
    Name(String),
}

/// Deskulpt widget manifest.
#[derive(Clone, Debug, Default, Deserialize, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    /// The display name of the widget.
    pub name: String,
    /// The version of the widget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = String)]
    pub version: Option<String>,
    /// The authors of the widget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = Vec<ManifestAuthor>)]
    pub authors: Option<Vec<ManifestAuthor>>,
    /// The license of the widget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = String)]
    pub license: Option<String>,
    /// A short description of the widget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = String)]
    pub description: Option<String>,
    /// URL to the homepage of the widget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = String)]
    pub homepage: Option<String>,
    /// The entry module of the widget that exports the widget component.
    ///
    /// This is a path relative to the root of the widget.
    #[serde(skip_serializing)]
    pub entry: String,
    /// Whether to ignore the widget.
    ///
    /// If set to true, the widget will not be discovered by the application,
    /// despite the presence of the manifest file.
    #[serde(default, skip_serializing)]
    pub ignore: bool,
}

impl Manifest {
    /// The name of the widget manifest file.
    ///
    /// A directory containing this file is considered a Deskulpt widget.
    const FILE_NAME: &str = "deskulpt.widget.json";

    /// Load the widget manifest from a directory.
    ///
    /// This method returns `Ok(None)` if the directory is **NOT A WIDGET**,
    /// i.e., either the directory does not contain a widget manifest file, or
    /// the widget manifest marks itself as ignored (see [`Self::ignore`]). If
    /// loading or parsing the widget manifest fails, an error is returned.
    /// Otherwise, the widget manifest is returned wrapped in `Ok(Some(...))`.
    ///
    /// Note that [`Result::transpose`] can bring `Option` out of `Result` for
    /// the result of this method, so that non-widget directories can be
    /// filtered out without nested pattern matching.
    pub fn load(dir: &Path) -> Result<Option<Self>> {
        let path = dir.join(Self::FILE_NAME);
        if !path.exists() {
            return Ok(None);
        }
        let file = File::open(&path)
            .with_context(|| format!("Failed to open widget manifest: {}", path.display()))?;
        let reader = BufReader::new(file);
        let config: Self = serde_json::from_reader(reader)
            .with_context(|| format!("Failed to parse widget manifest: {}", path.display()))?;
        if config.ignore {
            return Ok(None);
        }
        Ok(Some(config))
    }
}

/// The catalog of Deskulpt widgets.
///
/// This keeps a mapping from widget IDs to their manifests (if valid) or error
/// messages (if invalid).
#[derive(Clone, Debug, Default, Serialize, specta::Type)]
pub struct Catalog(pub BTreeMap<String, Outcome<Manifest>>);

impl Catalog {
    /// Load the widget catalog from a directory.
    ///
    /// This scans all top-level subdirectories and attempts to load them as
    /// widgets. Widget IDs are derived from the directory names. Widget
    /// manifests or error messages are stored accordingly, depending on
    /// whether the directory is successfully loaded as a widget. Non-widget
    /// directories are not included in the catalog.
    pub fn load(dir: &Path) -> Result<Self> {
        let mut catalog = Self::default();

        let entries = std::fs::read_dir(dir)?;
        for entry in entries {
            let entry = entry?;

            let path = entry.path();
            if !path.is_dir() {
                continue; // Non-directory entries are not widgets, skip
            }

            if let Some(manifest) = Manifest::load(&path).transpose() {
                // Since each widget must be at the top level of the widgets
                // directory, the directory names must be unique and we can use
                // them as widget IDs
                let id = entry.file_name().to_string_lossy().to_string();
                catalog.0.insert(id, manifest.into());
            }
        }

        Ok(catalog)
    }
}
