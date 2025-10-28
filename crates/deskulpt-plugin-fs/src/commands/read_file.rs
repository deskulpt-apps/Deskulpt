use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, TypedPluginCommand};
use serde::{Deserialize, Serialize};

pub struct ReadFile;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadFileInput {
    path: PathBuf,
}

#[derive(Serialize)]
pub struct ReadFileOutput {
    content: String,
}

impl TypedPluginCommand for ReadFile {
    type Input = ReadFileInput;
    type Output = ReadFileOutput;

    fn name(&self) -> &str {
        "read_file"
    }

    fn run_typed(
        &self,
        widget_id: &str,
        engine: &EngineInterface,
        input: Self::Input,
    ) -> Result<Self::Output> {
        let widget_dir = engine.widget_dir(widget_id)?;
        let file_path = widget_dir.join(input.path);

        let content = std::fs::read_to_string(&file_path)?;

        Ok(ReadFileOutput { content })
    }
}
