use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::call_plugin;
use deskulpt_plugin_fs::FsPlugin;
use serde_json::json;
use tempfile::TempDir;

fn create_widget_dir_fn(temp_path: PathBuf) -> impl Fn(&str) -> Result<PathBuf> + 'static {
    move |_id: &str| -> Result<PathBuf> { Ok(temp_path.clone()) }
}

#[test]
fn test_create_dir_single() {
    let temp_dir = TempDir::new().unwrap();
    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "new_dir"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "create_dir",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_ok());

    let dir_path = temp_dir.path().join("new_dir");
    assert!(dir_path.exists());
    assert!(dir_path.is_dir());
}

#[test]
fn test_create_dir_nested() {
    let temp_dir = TempDir::new().unwrap();
    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "level1/level2/level3"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "create_dir",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_ok());

    let dir_path = temp_dir.path().join("level1").join("level2").join("level3");
    assert!(dir_path.exists());
    assert!(dir_path.is_dir());
}

#[test]
fn test_create_dir_already_exists() {
    let temp_dir = TempDir::new().unwrap();
    let existing_dir = temp_dir.path().join("existing");
    std::fs::create_dir(&existing_dir).unwrap();

    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "existing"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "create_dir",
        "test_widget".to_string(),
        Some(input),
    );

    // create_dir_all doesn't error if directory exists
    assert!(result.is_ok());
    assert!(existing_dir.exists());
}
