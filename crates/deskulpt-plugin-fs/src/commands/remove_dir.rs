use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, TypedPluginCommand};
use serde::{Deserialize, Serialize};

pub struct RemoveDir;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDirInput {
    path: PathBuf,
    recursive: Option<bool>,
}

#[derive(Serialize)]
pub struct RemoveDirOutput {
    success: bool,
}

impl TypedPluginCommand for RemoveDir {
    type Input = RemoveDirInput;
    type Output = RemoveDirOutput;

    fn name(&self) -> &str {
        "remove_dir"
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
            std::fs::remove_dir_all(&dir_path)?;
        } else {
            std::fs::remove_dir(&dir_path)?;
        }

        Ok(RemoveDirOutput { success: true })
    }
}
