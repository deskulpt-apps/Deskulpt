use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, PluginCommand, dispatch};
use serde::Deserialize;

use crate::FsPlugin;

pub struct RemoveFile;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFileInputPayload {
    path: PathBuf,
}

impl PluginCommand for RemoveFile {
    type Plugin = FsPlugin;

    fn name(&self) -> &str {
        "remove_file"
    }

    #[dispatch]
    fn run(
        &self,
        id: String,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        input: RemoveFileInputPayload,
    ) -> Result<()> {
        let path = engine.widget_dir(&id).join(input.path);
        std::fs::remove_file(&path)?;
        Ok(())
    }
}
