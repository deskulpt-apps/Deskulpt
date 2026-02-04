use std::fs::File;
use std::io::BufWriter;

use anyhow::Result;
use schemars::schema_for;

pub fn run() -> Result<()> {
    let schemas = vec![("settings", schema_for!(deskulpt_settings::model::Settings))];

    let schema_dir = deskulpt_workspace::root_dir().join("resources/schema");
    for schema in schemas {
        let path = schema_dir.join(schema.0).with_extension("json");
        let file = File::create(&path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &schema.1)?;
        println!("✅ Generated: {}", path.display());
    }

    Ok(())
}
