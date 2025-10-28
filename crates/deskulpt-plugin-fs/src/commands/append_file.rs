use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, TypedPluginCommand};
use serde::{Deserialize, Serialize};

pub struct AppendFile;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppendFileInput {
    path: PathBuf,
    content: String,
}

#[derive(Serialize)]
pub struct AppendFileOutput {
    success: bool,
}

impl TypedPluginCommand for AppendFile {
    type Input = AppendFileInput;
    type Output = AppendFileOutput;

    fn name(&self) -> &str {
        "append_file"
    }

    fn run_typed(
        &self,
        widget_id: &str,
        engine: &EngineInterface,
        input: Self::Input,
    ) -> Result<Self::Output> {
        let widget_dir = engine.widget_dir(widget_id)?;
        let file_path = widget_dir.join(input.path);

        // Create parent directories if they don't exist
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)?
            .write_all(input.content.as_bytes())?;

        Ok(AppendFileOutput { success: true })
    }
}
