use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, PluginCommand, dispatch};
use serde::Deserialize;

use crate::FsPlugin;

pub struct IsDir;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsDirInputPayload {
    path: PathBuf,
}

impl PluginCommand for IsDir {
    type Plugin = FsPlugin;

    fn name(&self) -> &str {
        "is_dir"
    }

    #[dispatch]
    fn run(
        &self,
        id: String,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        input: IsDirInputPayload,
    ) -> Result<bool> {
        let path = engine.widget_dir(&id).join(input.path);
        Ok(path.is_dir())
    }
}
