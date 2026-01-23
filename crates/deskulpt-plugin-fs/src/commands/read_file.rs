use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, PluginCommand, dispatch};
use serde::Deserialize;

use crate::FsPlugin;

pub struct ReadFile;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadFileInputPayload {
    path: PathBuf,
}

impl PluginCommand for ReadFile {
    type Plugin = FsPlugin;

    fn name(&self) -> &str {
        "read_file"
    }

    #[dispatch]
    fn run(
        &self,
        id: String,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        input: ReadFileInputPayload,
    ) -> Result<String> {
        let path = engine.widget_dir(&id).join(input.path);
        let content = std::fs::read_to_string(&path)?;
        Ok(content)
    }
}
