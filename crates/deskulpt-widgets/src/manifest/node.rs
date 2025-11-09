//! Definition of the Node.js package manifest.

use std::collections::HashMap;

use serde::Deserialize;

use crate::manifest::LoadManifest;

/// The Node.js package manifest.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeManifest {
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

impl LoadManifest for NodeManifest {
    const FILE_NAME: &'static str = "package.json";
}
