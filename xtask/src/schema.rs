use anyhow::Result;
use schemars::schema_for;

pub fn run() -> Result<()> {
    let schemas = vec![("settings", schema_for!(deskulpt_settings::Settings))];

    let schema_dir = deskulpt_workspace::root_dir().join("resources/schema");
    for schema in schemas {
        let path = schema_dir.join(schema.0).with_extension("json");
        let content = serde_json::to_string(&schema.1)?;
        std::fs::write(&path, content + "\n")?;
        println!("✅ Generated: {}", path.display());
    }

    Ok(())
}
