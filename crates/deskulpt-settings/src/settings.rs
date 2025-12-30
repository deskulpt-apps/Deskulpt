//! Definitions, patching, and persistence of Deskulpt settings.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{DefaultOnError, MapSkipError, serde_as};

/// The light/dark theme of the application interface.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, JsonSchema, specta::Type,
)]
#[serde(rename_all = "camelCase")]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

/// The canvas interaction mode.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, JsonSchema, specta::Type,
)]
#[serde(rename_all = "camelCase")]
pub enum CanvasImode {
    /// Auto mode.
    ///
    /// Automatically switch between sink and float modes based on mouse
    /// position, so that users will feel like the widgets and the desktop are
    /// simultaneously interactable.
    #[default]
    Auto,
    /// Sink mode.
    ///
    /// The canvas is click-through. Widgets are not interactable. The desktop
    /// is interactable.
    Sink,
    /// Float mode.
    ///
    /// The canvas is not click-through. Widgets are interactable. The desktop
    /// is not interactable.
    Float,
}

/// Actions that can be bound to keyboard shortcuts.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, JsonSchema, specta::Type,
)]
#[serde(rename_all = "camelCase")]
pub enum ShortcutAction {
    /// Toggle the canvas interaction mode (imode).
    ToggleCanvasImode,
    /// Open the manager interface.
    OpenManager,
}

/// Per-widget settings.
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
}

/// Full settings of the Deskulpt application.
#[serde_as]
#[derive(Debug, Default, Deserialize, Serialize, JsonSchema, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    /// The application theme.
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub theme: Theme,
    /// The canvas interaction mode.
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub canvas_imode: CanvasImode,
    /// The keyboard shortcuts.
    ///
    /// This maps the actions to the shortcut strings that will trigger them.
    #[serde_as(deserialize_as = "MapSkipError<_, _>")]
    pub shortcuts: BTreeMap<ShortcutAction, String>,
    /// The mapping from widget IDs to their respective settings.
    #[serde_as(deserialize_as = "MapSkipError<_, _>")]
    pub widgets: BTreeMap<String, WidgetSettings>,
    /// Whether the starter widgets have been added.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[specta(skip)]
    pub starter_widgets_added: bool,
    /// Whether widget hot reload is enabled.
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub hot_reload_enabled: bool,
}

/// A patch for partial updates to [`Settings`].
#[derive(Debug, Default, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct SettingsPatch {
    /// If not `None`, update [`Settings::theme`].
    #[specta(optional, type = Theme)]
    pub theme: Option<Theme>,
    /// If not `None`, update [`Settings::canvas_imode`].
    #[specta(optional, type = CanvasImode)]
    pub canvas_imode: Option<CanvasImode>,
    /// If not `None`, update [`Settings::shortcuts`].
    ///
    /// Non-specified shortcuts will remain unchanged. If a shortcut value is
    /// `None`, it means removing that shortcut. Otherwise, it means updating
    /// or adding that shortcut.
    #[specta(optional, type = BTreeMap<ShortcutAction, Option<String>>)]
    pub shortcuts: Option<BTreeMap<ShortcutAction, Option<String>>>,
    /// If not `None`, update [`Settings::widgets`].
    ///
    /// Non-specified widgets will remain unchanged. If a widget settings patch
    /// is `None`, it means removing that widget. Otherwise, it means applying
    /// the patch to that widget settings. If the widget ID does not exist, a
    /// new widget settings will be created with default values, and then the
    /// patch will be applied to it.
    #[specta(optional, type = BTreeMap<String, Option<WidgetSettingsPatch>>)]
    pub widgets: Option<BTreeMap<String, Option<WidgetSettingsPatch>>>,
    /// If not `None`, update [`Settings::starter_widgets_added`].
    #[serde(skip)]
    pub starter_widgets_added: Option<bool>,
    /// If not `None`, update [`Settings::hot_reload_enabled`].
    #[specta(optional, type = bool)]
    pub hot_reload_enabled: Option<bool>,
}

impl Settings {
    /// Load the settings from disk.
    ///
    /// Default settings will be returned if the settings file does not exist.
    /// Corrupted settings file will attempt to recover as much data as
    /// possible, applying default values for the corrupted parts. However,
    /// if the file is completely corrupted, an error might still be returned.
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Default::default());
        }
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let settings: Settings = serde_json::from_reader(reader)?;
        Ok(settings)
    }

    /// Dump the settings to disk.
    ///
    /// The provided path will be created if it does not exist. The settings
    /// will be serialized in pretty JSON format with `$schema` metadata for
    /// human readability and editor support.
    pub fn dump(&self, path: &Path) -> Result<()> {
        #[derive(Serialize)]
        struct SettingsWithMeta<'a> {
            #[serde(rename = "$schema")]
            schema: &'static str,
            #[serde(flatten)]
            settings: &'a Settings,
        }

        const SETTINGS_SCHEMA_URL: &str =
            "https://deskulpt-apps.github.io/gen/settings-schema.json";

        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)?;
        }

        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let settings = SettingsWithMeta {
            schema: SETTINGS_SCHEMA_URL,
            settings: self,
        };
        serde_json::to_writer_pretty(writer, &settings)?;
        Ok(())
    }
}
