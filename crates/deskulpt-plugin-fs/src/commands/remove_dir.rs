use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, PluginCommand, dispatch};
use serde::Deserialize;

use crate::FsPlugin;

pub struct RemoveDir;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDirInputPayload {
    path: PathBuf,
}

impl PluginCommand for RemoveDir {
    type Plugin = FsPlugin;

    fn name(&self) -> &str {
        "remove_dir"
    }

    #[dispatch]
    fn run(
        &self,
        id: String,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        input: RemoveDirInputPayload,
    ) -> Result<()> {
        let path = engine.widget_dir(&id).join(input.path);
        std::fs::remove_dir_all(&path)?;
        Ok(())
    }
}
