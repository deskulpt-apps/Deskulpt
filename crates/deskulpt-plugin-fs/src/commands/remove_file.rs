use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, TypedPluginCommand};
use serde::{Deserialize, Serialize};

pub struct RemoveFile;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFileInput {
    path: PathBuf,
}

#[derive(Serialize)]
pub struct RemoveFileOutput {
    success: bool,
}

impl TypedPluginCommand for RemoveFile {
    type Input = RemoveFileInput;
    type Output = RemoveFileOutput;

    fn name(&self) -> &str {
        "remove_file"
    }

    fn run_typed(
        &self,
        widget_id: &str,
        engine: &EngineInterface,
        input: Self::Input,
    ) -> Result<Self::Output> {
        let widget_dir = engine.widget_dir(widget_id)?;
        let file_path = widget_dir.join(input.path);

        std::fs::remove_file(&file_path)?;

        Ok(RemoveFileOutput { success: true })
    }
}
