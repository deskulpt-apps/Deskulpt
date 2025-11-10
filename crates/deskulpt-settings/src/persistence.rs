//! Utilities for persisting the settings.

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Result;
use serde::Serialize;

use crate::Settings;

/// The URL to the JSON schema file of the settings.
static SETTINGS_SCHEMA_URL: &str = "https://deskulpt-apps.github.io/settings-schema.json";

/// Wrapper of [`Settings`] with additional metadata.
#[derive(Serialize)]
struct SettingsWithMeta<'a> {
    /// The JSON schema URL `$schema`.
    #[serde(rename = "$schema")]
    schema: &'static str,
    /// The settings.
    ///
    /// This field is borrowed because this struct is only for serialization
    /// purposes and does not need ownership so as to avoid unnecessary cloning.
    /// It is flattened in serialization.
    #[serde(flatten)]
    settings: &'a Settings,
}

impl<'a> SettingsWithMeta<'a> {
    /// Wrap the borrowed settings with metadata.
    fn new(settings: &'a Settings) -> Self {
        Self {
            schema: SETTINGS_SCHEMA_URL,
            settings,
        }
    }
}

impl Settings {
    /// Read the settings from the persistence directory.
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

    /// Write the settings to the persistence directory.
    pub fn dump(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)?;
        }

        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let settings = SettingsWithMeta::new(self);
        serde_json::to_writer_pretty(writer, &settings)?;
        Ok(())
    }
}
