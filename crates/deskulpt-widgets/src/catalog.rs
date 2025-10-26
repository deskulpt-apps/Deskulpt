//! The catalog of Deskulpt widgets.

use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use deskulpt_common::outcome::Outcome;
use deskulpt_core::settings::{Settings, SettingsPatch, WidgetSettingsPatch};
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
    pub ignore: bool,
}

/// The Node.js package manifest.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageManifest {
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

/// Helper trait for reading manifest files from a directory.
pub trait LoadManifest: Sized + DeserializeOwned {
    /// The name of the manifest file.
    const FILE_NAME: &'static str;

    /// Try to read the manifest from a directory.
    ///
    /// This method looks for [`Self::FILE_NAME`] in the given directory. It
    /// returns an error if any I/O or parsing error occurs. It returns
    /// `Ok(None)` if the file does not exist. Otherwise, it returns the
    /// parsed manifest wrapped in `Ok(Some(...))`.
    fn try_from_dir(dir: &Path) -> Result<Option<Self>> {
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

/// Full widget specification resolved from manifest files.
#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct WidgetSpec {
    /// The name of the widget.
    pub name: String,
    /// The entry point of the widget.
    pub entry: String,
    /// The dependencies of the widget.
    pub dependencies: HashMap<String, String>,
}

impl WidgetSpec {
    /// Try to construct widget specification from a directory.
    ///
    /// This method reads necessary manifest files from the given directory. In
    /// particular:
    ///
    /// - [`WidgetManifest`]: If this file does not exist or if it has `ignore`
    ///   set to true, the directory is not considered a widget, and the method
    ///   returns `Ok(None)`.
    /// - [`PackageManifest`]: This file is optional. If it exists, its contents
    ///   are merged into the final widget specification.
    ///
    /// If any error occurs while reading or merging the manifest files, the
    /// given directory is still considered a widget but invalid, and the error
    /// is propagated. On success, the merged widget specification is returned
    /// wrapped in `Ok(Some(...))`.
    pub fn try_from_dir(dir: &Path) -> Result<Option<Self>> {
        let widget_manifest = match WidgetManifest::try_from_dir(dir)
            .with_context(|| format!("Failed to load {}", WidgetManifest::FILE_NAME))?
        {
            Some(widget_manifest) if !widget_manifest.ignore => widget_manifest,
            _ => return Ok(None),
        };

        let package_manifest = PackageManifest::try_from_dir(dir)
            .with_context(|| format!("Failed to load {}", PackageManifest::FILE_NAME))?
            .unwrap_or_default();

        Ok(Some(WidgetSpec {
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

/// The catalog of widgets.
///
/// This is a mapping from widget IDs to their full specifications (if valid) or
/// error messages (if invalid).
#[derive(Debug, Default, Serialize, Deserialize, specta::Type)]
pub struct WidgetCatalog(pub BTreeMap<String, Outcome<WidgetSpec>>);

impl WidgetCatalog {
    /// Scan the widgets directory and construct the catalog.
    ///
    /// All top-level subdirectories of the given directory are looked into. If
    /// is not considered a widget, then it is skipped. Otherwise, either valid
    /// widget specification or an error message is recorded in the catalog. The
    /// directory names are used as widget IDs.
    pub fn scan(root: &Path) -> Result<Self> {
        let mut catalog = Self::default();

        let entries = std::fs::read_dir(root)?;
        for entry in entries {
            let entry = entry?;

            let path = entry.path();
            if !path.is_dir() {
                continue; // Non-directory entries are not widgets, skip
            }

            if let Some(config) = WidgetSpec::try_from_dir(&path)
                .map(|opt| opt.map(Outcome::Ok))
                .unwrap_or_else(|e| Some(Outcome::Err(format!("{e:?}"))))
            {
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
        let mut patches = HashMap::new();

        for e in itertools::merge_join_by(
            settings.widgets.iter(), // Old
            self.0.iter(),           // New
            |(a, _), (b, _)| a.cmp(b),
        ) {
            match e {
                itertools::EitherOrBoth::Left((id, _)) => {
                    patches.insert(id.clone(), None);
                },
                itertools::EitherOrBoth::Right((id, spec)) => {
                    let patch = match spec {
                        Outcome::Ok(spec) => spec.compute_settings_patch(),
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
