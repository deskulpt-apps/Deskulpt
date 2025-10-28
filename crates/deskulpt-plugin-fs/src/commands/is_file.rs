use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, TypedPluginCommand};
use serde::{Deserialize, Serialize};

pub struct IsFile;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsFileInput {
    path: PathBuf,
}

#[derive(Serialize)]
pub struct IsFileOutput {
    is_file: bool,
}

impl TypedPluginCommand for IsFile {
    type Input = IsFileInput;
    type Output = IsFileOutput;

    fn name(&self) -> &str {
        "is_file"
    }

    fn run_typed(
        &self,
        widget_id: &str,
        engine: &EngineInterface,
        input: Self::Input,
    ) -> Result<Self::Output> {
        let widget_dir = engine.widget_dir(widget_id)?;
        let file_path = widget_dir.join(input.path);

        let is_file = file_path.is_file();

        Ok(IsFileOutput { is_file })
    }
}
