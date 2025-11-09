use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use serde::de::DeserializeOwned;

/// Helper trait for loading manifest files from a directory.
pub trait LoadManifest: Sized + DeserializeOwned {
    /// The name of the manifest file to look for in the directory.
    const FILE_NAME: &'static str;

    /// Load the manifest file from the given directory.
    ///
    /// This method does not treat the absence of the manifest file as an error.
    /// Instead, it returns `Ok(None)` in that case. If the file exists but
    /// cannot be read or parsed, an error is returned. Otherwise, the parsed
    /// manifest is returned wrapped in `Ok(Some(...))`.
    fn load(dir: &Path) -> Result<Option<Self>> {
        let path = dir.join(Self::FILE_NAME);
        if !path.exists() {
            return Ok(None);
        }
        let file = File::open(&path)
            .with_context(|| format!("Failed to open manifest file: {}", path.display()))?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)
            .with_context(|| format!("Failed to parse manifest file: {}", path.display()))?;
        Ok(Some(config))
    }
}
