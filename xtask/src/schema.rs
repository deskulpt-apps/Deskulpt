use anyhow::Result;
use schemars::schema_for;

pub fn run() -> Result<()> {
    let schema = schema_for!(deskulpt_settings::Settings);
    let output = serde_json::to_string_pretty(&schema)?;

    let path = deskulpt_workspace::root_dir().join("public/gen/settings-schema.json");
    std::fs::write(&path, output)?;
    println!("✅ Generated: {}", path.display());

    Ok(())
}
