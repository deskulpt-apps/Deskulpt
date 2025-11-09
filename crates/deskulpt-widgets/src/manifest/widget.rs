//! Definition of the Deskulpt widget manifest.

use serde::Deserialize;

use crate::manifest::LoadManifest;

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

impl LoadManifest for WidgetManifest {
    const FILE_NAME: &'static str = "deskulpt.widget.json";
}
