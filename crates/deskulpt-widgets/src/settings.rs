use anyhow::Result;
use deskulpt_manifest::WidgetManifest;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{DefaultOnError, serde_as};

/// Per-widget settings.
#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, specta::Type)]
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

/// A patch for partial updates to [`Settings`].
#[derive(Debug, Default, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct WidgetSettingsPatch {
    /// If not `None`, update [`Settings::x`].
    #[specta(optional, type = i32)]
    pub x: Option<i32>,
    /// If not `None`, update [`Settings::y`].
    #[specta(optional, type = i32)]
    pub y: Option<i32>,
    /// If not `None`, update [`Settings::width`].
    #[specta(optional, type = u32)]
    pub width: Option<u32>,
    /// If not `None`, update [`Settings::height`].
    #[specta(optional, type = u32)]
    pub height: Option<u32>,
    /// If not `None`, update [`Settings::opacity`].
    #[specta(optional, type = u8)]
    pub opacity: Option<u8>,
    /// If not `None`, update [`Settings::z_index`].
    #[specta(optional, type = i16)]
    pub z_index: Option<i16>,
    /// If not `None`, update [`Settings::is_loaded`].
    #[specta(optional, type = bool)]
    pub is_loaded: Option<bool>,
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

    pub fn from_manifest(_manifest: &WidgetManifest) -> Self {
        // TODO: Implement when we support default settings in the manifest
        Self {
            ..Default::default()
        }
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
}
