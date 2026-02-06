//! Deskulpt widget manifest and catalog.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use deskulpt_common::outcome::Outcome;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{DefaultOnError, serde_as};

/// An author of a Deskulpt widget.
#[derive(Debug, Deserialize, Serialize, specta::Type)]
#[serde(untagged)]
pub enum WidgetManifestAuthor {
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
#[derive(Debug, Default, Deserialize, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct WidgetManifest {
    /// The display name of the widget.
    pub name: String,
    /// The version of the widget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = String)]
    pub version: Option<String>,
    /// The authors of the widget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = Vec<WidgetManifestAuthor>)]
    pub authors: Option<Vec<WidgetManifestAuthor>>,
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

impl WidgetManifest {
    /// The name of the widget manifest file.
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
    fn load(dir: &Path) -> Result<Option<Self>> {
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

/// Deskulpt widget settings.
#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct WidgetSettings {
    /// The leftmost x-coordinate in pixels.
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub x: i32,
    /// The topmost y-coordinate in pixels.
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub y: i32,
    /// The width in pixels.
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub width: u32,
    /// The height in pixels.
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub height: u32,
    /// The opacity in percentage.
    #[serde(deserialize_with = "WidgetSettings::deserialize_opacity")]
    #[schemars(range(min = 1, max = 100))]
    pub opacity: u8,
    /// The z-index.
    ///
    /// Higher z-index means the widget will be rendered above those with lower
    /// z-index. Widgets with the same z-index can have arbitrary rendering
    /// order. The allowed range is from -999 to 999.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(range(min = -999, max = 999))]
    pub z_index: i16,
    /// Whether the widget should be loaded on the canvas or not.
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub is_loaded: bool,
}

impl Default for WidgetSettings {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 300,
            height: 200,
            opacity: 100,
            z_index: 0,
            is_loaded: true,
        }
    }
}

/// A patch for partial updates to [`WidgetSettings`].
#[derive(Debug, Default, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct WidgetSettingsPatch {
    /// If not `None`, update [`WidgetSettings::x`].
    #[specta(optional, type = i32)]
    pub x: Option<i32>,
    /// If not `None`, update [`WidgetSettings::y`].
    #[specta(optional, type = i32)]
    pub y: Option<i32>,
    /// If not `None`, update [`WidgetSettings::width`].
    #[specta(optional, type = u32)]
    pub width: Option<u32>,
    /// If not `None`, update [`WidgetSettings::height`].
    #[specta(optional, type = u32)]
    pub height: Option<u32>,
    /// If not `None`, update [`WidgetSettings::opacity`].
    #[specta(optional, type = u8)]
    pub opacity: Option<u8>,
    /// If not `None`, update [`WidgetSettings::z_index`].
    #[specta(optional, type = i16)]
    pub z_index: Option<i16>,
    /// If not `None`, update [`WidgetSettings::is_loaded`].
    #[specta(optional, type = bool)]
    pub is_loaded: Option<bool>,
}

impl WidgetSettings {
    /// Deserialization helper for opacity.
    ///
    /// On error deserializing this field, it will be set to default (100). The
    /// deserialized value will be clamped to [1, 100].
    fn deserialize_opacity<'de, D>(deserializer: D) -> Result<u8, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u8::deserialize(deserializer) {
            Ok(opacity) => Ok(opacity.clamp(1, 100)),
            Err(_) => Ok(100),
        }
    }

    /// Derive widget settings from a widget manifest.
    ///
    /// NOTE: Currently this just returns default settings, but in the future
    /// when the manifest have fields that can imply default settings, this
    /// method should derive settings from those fields.
    fn from_manifest(_manifest: &WidgetManifest) -> Self {
        Self::default()
    }

    /// Apply a [`WidgetSettingsPatch`].
    ///
    /// This method also returns whether the widget settings is actually changed
    /// by the patch.
    pub fn apply_patch(&mut self, patch: WidgetSettingsPatch) -> bool {
        #[inline]
        fn set_if_changed<T: PartialEq>(dst: &mut T, src: Option<T>) -> bool {
            match src {
                Some(v) if *dst != v => {
                    *dst = v;
                    true
                },
                _ => false,
            }
        }

        let mut dirty = false;
        dirty |= set_if_changed(&mut self.x, patch.x);
        dirty |= set_if_changed(&mut self.y, patch.y);
        dirty |= set_if_changed(&mut self.width, patch.width);
        dirty |= set_if_changed(&mut self.height, patch.height);
        dirty |= set_if_changed(&mut self.opacity, patch.opacity);
        dirty |= set_if_changed(&mut self.z_index, patch.z_index);
        dirty |= set_if_changed(&mut self.is_loaded, patch.is_loaded);
        dirty
    }

    /// Check if the widget covers the given point geometrically.
    ///
    /// Note that all edges are inclusive.
    pub fn covers_point(&self, x: f64, y: f64) -> bool {
        let sx = self.x as f64;
        let sy = self.y as f64;
        let ex = sx + self.width as f64;
        let ey = sy + self.height as f64;

        x >= sx && x <= ex && y >= sy && y <= ey
    }
}

/// A Deskulpt widget.
#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Widget {
    /// The manifest of the widget or an error message loading it.
    pub manifest: Outcome<WidgetManifest>,
    /// The settings of the widget.
    pub settings: WidgetSettings,
}

impl Widget {
    /// Create a new [`Widget`] instance.
    ///
    /// If settings are not provided, they will be derived from the manifest or
    /// set to default.
    fn new(manifest: Outcome<WidgetManifest>, settings: Option<WidgetSettings>) -> Self {
        let settings = settings.unwrap_or_else(|| match &manifest {
            Outcome::Ok(manifest) => WidgetSettings::from_manifest(manifest),
            Outcome::Err(_) => WidgetSettings::default(),
        });
        Self { manifest, settings }
    }
}

/// The catalog of Deskulpt widgets.
#[derive(Debug, Default, Serialize, specta::Type)]
pub struct WidgetCatalog(pub BTreeMap<String, Widget>);

impl WidgetCatalog {
    /// Reload a widget in the catalog from its directory.
    ///
    /// If the widget is gone, it will be removed from the catalog. If the
    /// widget is new, it will be added to the catalog with default settings. If
    /// the widget already exists, its manifest will be updated while keeping
    /// its settings.
    pub fn reload(&mut self, dir: &Path, id: &str) -> Result<()> {
        let Some(manifest) = WidgetManifest::load(dir).transpose() else {
            self.0.remove(id);
            return Ok(());
        };

        if let Some(widget) = self.0.get_mut(id) {
            widget.manifest = manifest.into();
        } else {
            let widget = Widget::new(manifest.into(), None);
            self.0.insert(id.to_string(), widget);
        }

        Ok(())
    }

    /// Reload all widgets from the given directory.
    ///
    /// This will completely replace the current catalog with the widgets
    /// discovered in the given directory. Existing widgets will keep their
    /// settings if they are still present.
    pub fn reload_all(&mut self, dir: &Path) -> Result<()> {
        let mut new_catalog = Self::default();

        let entries = std::fs::read_dir(dir)?;
        for entry in entries {
            let entry = entry?;

            let path = entry.path();
            if !path.is_dir() {
                continue; // Non-directory entries are not widgets, skip
            }

            let Some(manifest) = WidgetManifest::load(&path).transpose() else {
                continue; // Not a widget, skip
            };

            // Since each widget must be at the top level of the widgets
            // directory, the directory names must be unique and we can use them
            // as widget IDs
            let id = entry.file_name().to_string_lossy().to_string();

            let settings = self.0.remove(&id).map(|w| w.settings);
            let widget = Widget::new(manifest.into(), settings);
            new_catalog.0.insert(id, widget);
        }

        *self = new_catalog;
        Ok(())
    }
}
