use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, PluginCommand, dispatch};
use serde::Deserialize;

use crate::FsPlugin;

pub struct IsFile;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsFileInputPayload {
    path: PathBuf,
}

impl PluginCommand for IsFile {
    type Plugin = FsPlugin;

    fn name(&self) -> &str {
        "is_file"
    }

    #[dispatch]
    fn run(
        &self,
        id: String,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        input: IsFileInputPayload,
    ) -> Result<bool> {
        let path = engine.widget_dir(id.as_str())?.join(input.path);
        Ok(path.is_file())
    }
}
