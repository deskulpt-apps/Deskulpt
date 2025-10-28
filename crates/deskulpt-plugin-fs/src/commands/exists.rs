use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, TypedPluginCommand};
use serde::{Deserialize, Serialize};

pub struct Exists;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExistsInput {
    path: PathBuf,
}

#[derive(Serialize)]
pub struct ExistsOutput {
    exists: bool,
}

impl TypedPluginCommand for Exists {
    type Input = ExistsInput;
    type Output = ExistsOutput;

    fn name(&self) -> &str {
        "exists"
    }

    fn run_typed(
        &self,
        widget_id: &str,
        engine: &EngineInterface,
        input: Self::Input,
    ) -> Result<Self::Output> {
        let widget_dir = engine.widget_dir(widget_id)?;
        let file_path = widget_dir.join(input.path);

        let exists = file_path.exists();

        Ok(ExistsOutput { exists })
    }
}
