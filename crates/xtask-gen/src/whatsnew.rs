use anyhow::{bail, Result};
use clap::Parser;
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Debug, Parser, Serialize)]
pub struct Args {
    /// The version to generate for, in the format "vX.Y.Z".
    version: String,
}

pub fn run(args: Args) -> Result<()> {
    let dir = deskulpt_workspace::docs_dir().join("whatsnew");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.md", args.version));
    if path.exists() {
        bail!("What's New file already exists for {}", args.version);
    }

    let mut hb = Handlebars::new();
    hb.register_escape_fn(handlebars::no_escape);
    hb.register_template_string("whatsnew", include_str!("whatsnew.md.hbs"))?;

    let whatsnew = hb.render("whatsnew", &args)?;
    std::fs::write(&path, whatsnew)?;
    println!("✅ Generated: {}", path.display());

    Ok(())
}
