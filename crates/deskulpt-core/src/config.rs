//! Configuration of Deskulpt widgets.

use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use deskulpt_common::outcome::Outcome;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::settings::{Settings, SettingsPatch, WidgetSettingsPatch};

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

/// Full configuration of a Deskulpt widget.
#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct WidgetConfig {
    /// The name of the widget.
    pub name: String,
    /// The entry point of the widget.
    pub entry: String,
    /// The dependencies of the widget.
    pub dependencies: HashMap<String, String>,
}

impl WidgetConfig {
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

        Ok(Some(WidgetConfig {
            name: widget_manifest.name,
            entry: widget_manifest.entry,
            dependencies: package_manifest.dependencies,
        }))
    }

    /// Compute a widget settings patch from the specification.
    ///
    /// TODO: Currently this method always returns an empty patch. In the
    /// future, we will extend the widget manifest to allow customization of
    /// default widget settings, which will be used to compute the patch.
    pub fn compute_settings_patch(&self) -> WidgetSettingsPatch {
        Default::default()
    }
}

/// The widget catalog.
///
/// This is a collection of all widgets discovered locally, mapped from their
/// widget IDs to their configurations. Invalid widgets are also included with
/// their error messages.
#[derive(Debug, Default, Serialize, Deserialize, specta::Type)]
pub struct WidgetCatalog(pub BTreeMap<String, Outcome<WidgetConfig>>);

impl WidgetCatalog {
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

            if let Some(config) = WidgetConfig::load(&path)
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

    /// Compute a settings patch to synchronize with the current catalog.
    ///
    /// This method compares the current catalog with the given (old) settings
    /// and generates a settings patch that:
    ///
    /// - If a widget exists in the old settings but not in the current catalog,
    ///   it will be removed.
    /// - If a widget exists in the current catalog but not in the old settings,
    ///   it will be added. If the widget is valid, an appropriate patch is
    ///   computed with [`WidgetSpec::compute_settings_patch`]. Otherwise, an
    ///   empty patch is used. In either case, fields not specified in the patch
    ///   will be filled with default when the patch is actually applied.
    /// - If a widget exists in both, no changes are made.
    pub fn compute_settings_patch(&self, settings: &Settings) -> SettingsPatch {
        let mut patches = BTreeMap::new();

        for e in itertools::merge_join_by(
            settings.widgets.iter(), // Old
            self.0.iter(),           // New
            |(a, _), (b, _)| a.cmp(b),
        ) {
            match e {
                itertools::EitherOrBoth::Left((id, _)) => {
                    patches.insert(id.clone(), None);
                },
                itertools::EitherOrBoth::Right((id, config)) => {
                    let patch = match config {
                        Outcome::Ok(config) => config.compute_settings_patch(),
                        Outcome::Err(_) => Default::default(),
                    };
                    patches.insert(id.clone(), Some(patch));
                },
                itertools::EitherOrBoth::Both(_, _) => {},
            }
        }

        SettingsPatch {
            widgets: Some(patches),
            ..Default::default()
        }
    }
}
