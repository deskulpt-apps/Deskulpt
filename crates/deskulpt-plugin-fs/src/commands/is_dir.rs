use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, TypedPluginCommand};
use serde::{Deserialize, Serialize};

pub struct IsDir;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsDirInput {
    path: PathBuf,
}

#[derive(Serialize)]
pub struct IsDirOutput {
    is_dir: bool,
}

impl TypedPluginCommand for IsDir {
    type Input = IsDirInput;
    type Output = IsDirOutput;

    fn name(&self) -> &str {
        "is_dir"
    }

    fn run_typed(
        &self,
        widget_id: &str,
        engine: &EngineInterface,
        input: Self::Input,
    ) -> Result<Self::Output> {
        let widget_dir = engine.widget_dir(widget_id)?;
        let file_path = widget_dir.join(input.path);

        let is_dir = file_path.is_dir();

        Ok(IsDirOutput { is_dir })
    }
}
