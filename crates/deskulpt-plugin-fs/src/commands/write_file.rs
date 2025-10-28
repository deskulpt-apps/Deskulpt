use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, TypedPluginCommand};
use serde::{Deserialize, Serialize};

pub struct WriteFile;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteFileInput {
    path: PathBuf,
    content: String,
}

#[derive(Serialize)]
pub struct WriteFileOutput {
    success: bool,
}

impl TypedPluginCommand for WriteFile {
    type Input = WriteFileInput;
    type Output = WriteFileOutput;

    fn name(&self) -> &str {
        "write_file"
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

        std::fs::write(&file_path, input.content)?;

        Ok(WriteFileOutput { success: true })
    }
}
