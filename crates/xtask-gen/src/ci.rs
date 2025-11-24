use std::process::Command;

use anyhow::{Result, bail};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Skip build checks.
    #[arg(long)]
    pub skip_build: bool,
}

pub fn run(args: Args) -> Result<()> {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("▶ CI Hygiene Checks");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    run_check("Formatting check", "pnpm", &["format:check"])?;
    run_check("Unused Node dependencies check", "pnpm", &["knip"])?;

    if has_command("cargo-shear") {
        run_check("Unused Rust dependencies check", "cargo", &["shear"])?;
    } else {
        println!(
            "\x1b[33m⚠ cargo-shear not found. Install it with: cargo install cargo-shear\x1b[0m"
        );
        println!("\x1b[33mSkipping unused Rust dependencies check...\x1b[0m");
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("▶ Auto-generated Files Check");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    println!("\x1b[33mRegenerating auto-generated files...\x1b[0m");
    run_command("pnpm", &["build:packages"])?;
    run_command("cargo", &["gen", "bindings"])?;
    run_command("cargo", &["gen", "schema"])?;

    let status_output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()?;

    if !status_output.stdout.is_empty() {
        println!("\x1b[31m✗ Auto-generated files are out of date\x1b[0m");
        println!("\nUncommitted changes detected:");
        run_command("git", &["status", "--porcelain"])?;
        println!("\nDiff:");
        let _ = Command::new("git").args(["--no-pager", "diff"]).status();
        println!("\nTo fix this issue, run the following commands and commit the changes:");
        println!("  pnpm build:packages");
        println!("  cargo gen bindings");
        println!("  cargo gen schema");
        bail!("Auto-generated files are out of date");
    } else {
        println!("\x1b[32m✓ Auto-generated files are up to date\x1b[0m");
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("▶ Linting Checks");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    run_check("Linting check", "pnpm", &["lint:check"])?;

    if !args.skip_build {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("▶ Build Checks");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        run_check("Package build", "pnpm", &["--filter", "deskulpt", "build"])?;
        run_check(
            "Rust check (type checking)",
            "cargo",
            &["check", "--workspace"],
        )?;
        run_check("Rust build", "cargo", &["build", "--bin", "deskulpt"])?;
    } else {
        println!("\x1b[33m⚠ Skipping build checks (--skip-build flag)\x1b[0m");
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\x1b[32m✓ All CI checks passed!\x1b[0m");
    println!("\x1b[32mYour changes should pass GitHub Actions CI.\x1b[0m");

    Ok(())
}

fn run_check(name: &str, cmd: &str, args: &[&str]) -> Result<()> {
    println!("\x1b[33mRunning: {} {}\x1b[0m", cmd, args.join(" "));
    let status = Command::new(cmd).args(args).status()?;

    if status.success() {
        println!("\x1b[32m✓ {} passed\x1b[0m", name);
        Ok(())
    } else {
        println!("\x1b[31m✗ {} failed\x1b[0m", name);
        bail!("{} failed", name);
    }
}

fn run_command(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd).args(args).status()?;
    if !status.success() {
        bail!("Command failed: {} {}", cmd, args.join(" "));
    }
    Ok(())
}

fn has_command(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
