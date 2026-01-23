use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, PluginCommand, dispatch};
use serde::Deserialize;

use crate::FsPlugin;

pub struct AppendFile;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppendFileInputPayload {
    path: PathBuf,
    content: String,
}

impl PluginCommand for AppendFile {
    type Plugin = FsPlugin;

    fn name(&self) -> &str {
        "append_file"
    }

    #[dispatch]
    fn run(
        &self,
        id: String,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        input: AppendFileInputPayload,
    ) -> Result<()> {
        let path = engine.widget_dir(&id).join(input.path);
        let mut file = std::fs::OpenOptions::new().append(true).open(&path)?;
        file.write_all(input.content.as_bytes())?;
        Ok(())
    }
}
