//! Application and widget settings.

use std::collections::{BTreeMap, HashMap};

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{serde_as, DefaultOnError, MapSkipError};

mod persistence;
mod shortcuts;

/// Light/dark theme of the application.
#[derive(Debug, Clone, Default, Deserialize, Serialize, JsonSchema, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

/// Types of keyboard shortcuts in the application.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    JsonSchema,
    specta::Type,
)]
#[serde(rename_all = "camelCase")]
pub enum ShortcutKey {
    /// For toggling canvas interaction mode.
    ToggleCanvasImode,
    /// For opening the manager window.
    OpenManager,
}

/// Per-widget settings.
///
/// Different from widget configurations, these are independent of the widget
/// configuration files and are managed internally by the application.
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
}

impl Default for WidgetSettings {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 300,
            height: 200,
            opacity: 100,
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
}

/// Full settings of the Deskulpt application.
#[serde_as]
#[derive(Debug, Default, Deserialize, Serialize, JsonSchema, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    /// The application theme.
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub theme: Theme,
    /// The keyboard shortcuts.
    #[serde_as(deserialize_as = "MapSkipError<_, _>")]
    pub shortcuts: BTreeMap<ShortcutKey, String>,
    /// The mapping from widget IDs to their respective settings.
    #[serde_as(deserialize_as = "MapSkipError<_, _>")]
    pub widgets: BTreeMap<String, WidgetSettings>,
}

/// A patch for partial updates to [`Settings`].
#[derive(Debug, Default, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct SettingsPatch {
    /// If not `None`, update [`Settings::theme`].
    #[specta(optional, type = Theme)]
    pub theme: Option<Theme>,
    /// If not `None`, update [`Settings::shortcuts`].
    ///
    /// Non-specified shortcuts will remain unchanged. If a shortcut value is
    /// `None`, it means removing that shortcut. Otherwise, it means updating
    /// or adding that shortcut.
    #[specta(optional, type = HashMap<ShortcutKey, Option<String>>)]
    pub shortcuts: Option<HashMap<ShortcutKey, Option<String>>>,
    /// If not `None`, update [`Settings::widgets`].
    ///
    /// Non-specified widgets will remain unchanged. If a widget settings patch
    /// is `None`, it means leaving that widget settings unchanged. Otherwise,
    /// it means applying the patch to that widget settings. If the widget ID
    /// does not exist, a new widget settings will be created with default
    /// values, and then the patch will be applied to it.
    #[specta(optional, type = HashMap<String, Option<WidgetSettingsPatch>>)]
    pub widgets: Option<HashMap<String, Option<WidgetSettingsPatch>>>,
}
