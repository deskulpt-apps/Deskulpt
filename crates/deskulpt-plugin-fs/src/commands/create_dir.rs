use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, PluginCommand, dispatch};
use serde::Deserialize;

use crate::FsPlugin;

pub struct CreateDir;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDirInputPayload {
    path: PathBuf,
}

impl PluginCommand for CreateDir {
    type Plugin = FsPlugin;

    fn name(&self) -> &str {
        "create_dir"
    }

    #[dispatch]
    fn run(
        &self,
        id: String,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        input: CreateDirInputPayload,
    ) -> Result<()> {
        let path = engine.widget_dir(id.as_str())?.join(input.path);
        std::fs::create_dir_all(&path)?;
        Ok(())
    }
}
