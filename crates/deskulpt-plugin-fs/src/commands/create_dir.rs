use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, TypedPluginCommand};
use serde::{Deserialize, Serialize};

pub struct CreateDir;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDirInput {
    path: PathBuf,
    recursive: Option<bool>,
}

#[derive(Serialize)]
pub struct CreateDirOutput {
    success: bool,
}

impl TypedPluginCommand for CreateDir {
    type Input = CreateDirInput;
    type Output = CreateDirOutput;

    fn name(&self) -> &str {
        "create_dir"
    }

    fn run_typed(
        &self,
        widget_id: &str,
        engine: &EngineInterface,
        input: Self::Input,
    ) -> Result<Self::Output> {
        let widget_dir = engine.widget_dir(widget_id)?;
        let dir_path = widget_dir.join(input.path);

        if input.recursive.unwrap_or(false) {
            std::fs::create_dir_all(&dir_path)?;
        } else {
            std::fs::create_dir(&dir_path)?;
        }

        Ok(CreateDirOutput { success: true })
    }
}
