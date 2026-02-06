//! Definitions, patching, and persistence of Deskulpt settings.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
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
    /// Open Deskulpt portal.
    OpenPortal,
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
    /// Whether the starter widgets have been added.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[specta(skip)]
    pub starter_widgets_added: bool,
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
    /// If not `None`, update [`Settings::starter_widgets_added`].
    #[serde(skip)]
    pub starter_widgets_added: Option<bool>,
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
    pub fn dump(&self, path: &Path, schema_url: &str) -> Result<()> {
        #[derive(Serialize)]
        struct SettingsWithMeta<'a> {
            #[serde(rename = "$schema")]
            schema: &'a str,
            #[serde(flatten)]
            settings: &'a Settings,
        }

        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)?;
        }

        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let settings = SettingsWithMeta {
            schema: schema_url,
            settings: self,
        };
        serde_json::to_writer_pretty(writer, &settings)?;
        Ok(())
    }
}
