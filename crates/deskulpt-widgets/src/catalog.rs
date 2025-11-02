//! Catalog for Deskulpt widgets.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use anyhow::{Context, Result};
use deskulpt_common::outcome::Outcome;
use deskulpt_core::settings::{Settings, SettingsPatch};
use serde::Serialize;

use crate::catalog::manifests::{LoadManifest, PackageManifest, WidgetManifest};

mod manifests;

/// Deskulpt widget descriptor.
///
/// This contains widget metadata obtained from manifest files necessary for
/// bundling and rendering the widget.
#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct WidgetDescriptor {
    /// The name of the widget.
    pub name: String,
    /// The entry point of the widget.
    pub entry: String,
    /// The dependencies of the widget.
    pub dependencies: HashMap<String, String>,
}

impl WidgetDescriptor {
    /// Load the widget descriptor from a directory.
    ///
    /// Specially, this method returns `Ok(None)` if the directory does not
    /// contain a widget descriptor file or if the widget is explicitly marked
    /// as ignored in the widget manifest.
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

        Ok(Some(WidgetDescriptor {
            name: widget_manifest.name,
            entry: widget_manifest.entry,
            dependencies: package_manifest.dependencies,
        }))
    }
}

/// The catalog of Deskulpt widgets.
///
/// This keeps a mapping from widget IDs to their descriptors (if valid) or
/// error messages (if invalid).
#[derive(Debug, Default, Serialize, specta::Type)]
pub struct WidgetCatalog(pub BTreeMap<String, Outcome<WidgetDescriptor>>);

impl WidgetCatalog {
    /// Load the widget catalog from a directory.
    ///
    /// This scans all top-level subdirectories and attempts to load them as
    /// widgets. Widget IDs are derived from the directory names. Widget
    /// descriptors or error messages are stored accordingly, depending on
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

            if let Some(descriptor) = WidgetDescriptor::load(&path)
                .map(|opt| opt.map(Outcome::Ok))
                .unwrap_or_else(|e| Some(Outcome::Err(format!("{e:?}"))))
            {
                // Since each widget must be at the top level of the widgets
                // directory, the directory names must be unique and we can use
                // them as widget IDs
                let id = entry.file_name().to_string_lossy().to_string();
                catalog.0.insert(id, descriptor);
            }
        }

        Ok(catalog)
    }

    /// Compute a settings patch to synchronize with the catalog.
    ///
    /// This method compares the given widget settings with catalog and
    /// generates a patch such that:
    ///
    /// - If a widget exists in the settings but not in the catalog, it will be
    ///   removed from the settings.
    /// - If a widget exists in the catalog but not in the settings, it will be
    ///   added to the settings with an empty patch, which results in default
    ///   settings.
    /// - If a widget exists in both, no changes are made.
    pub fn compute_settings_patch(&self, settings: &Settings) -> SettingsPatch {
        let mut patches = BTreeMap::new();

        for e in itertools::merge_join_by(
            settings.widgets.iter(), // settings (to be synced)
            self.0.iter(),           // catalog (truth)
            |(a, _), (b, _)| a.cmp(b),
        ) {
            match e {
                itertools::EitherOrBoth::Left((id, _)) => {
                    patches.insert(id.clone(), None);
                },
                itertools::EitherOrBoth::Right((id, _)) => {
                    patches.insert(id.clone(), Some(Default::default()));
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
