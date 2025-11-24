mod bindings;
mod ci;
mod schema;
mod whatsnew;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate Deskulpt frontend bindings.
    Bindings,
    /// Generate JSON schemas.
    Schema,
    /// Run local CI checks (same as GitHub Actions CI).
    Ci(ci::Args),
    /// Generate a "What's New" template for the next release.
    Whatsnew(whatsnew::Args),
}

/// [XTASK] Code generation for Deskulpt.
#[derive(Debug, Parser)]
#[command(version, about, author, bin_name = "cargo gen")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Commands::Bindings => bindings::run()?,
        Commands::Schema => schema::run()?,
        Commands::Ci(args) => ci::run(args)?,
        Commands::Whatsnew(args) => whatsnew::run(args)?,
    }
    Ok(())
}
