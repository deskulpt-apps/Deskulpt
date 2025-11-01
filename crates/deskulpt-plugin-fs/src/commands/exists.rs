use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, PluginCommand, dispatch};
use serde::Deserialize;

use crate::FsPlugin;

pub struct Exists;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExistsInputPayload {
    path: PathBuf,
}

impl PluginCommand for Exists {
    type Plugin = FsPlugin;

    fn name(&self) -> &str {
        "exists"
    }

    #[dispatch]
    fn run(
        &self,
        id: String,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        input: ExistsInputPayload,
    ) -> Result<bool> {
        let path = engine.widget_dir(&id)?.join(input.path);
        Ok(path.exists())
    }
}
