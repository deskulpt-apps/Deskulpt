#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg",
    html_favicon_url = "https://github.com/deskulpt-apps/Deskulpt/raw/main/public/deskulpt.svg"
)]

use std::path::PathBuf;

/// Get the root directory of the workspace.
pub fn root_dir() -> PathBuf {
    PathBuf::from(env!("WORKSPACE_DIR"))
}

/// Get the docs directory of the workspace.
pub fn docs_dir() -> PathBuf {
    root_dir().join("docs")
}

/// Get the directory of a crate in the workspace.
pub fn crate_dir(crate_name: &str) -> PathBuf {
    root_dir().join("crates").join(crate_name)
}

/// Get the directory of a package in the workspace.
pub fn package_dir(package_name: &str) -> PathBuf {
    root_dir().join("packages").join(package_name)
}

#[test]
fn test_root_dir() {
    let root_dir = root_dir();
    assert!(
        root_dir.join("pnpm-workspace.yaml").exists(),
        "Incorrect root dir: {root_dir:?}; expected to contain pnpm-workspace.yaml"
    );
}
