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
fn test_remove_dir_empty() {
    let temp_dir = TempDir::new().unwrap();
    let test_dir = temp_dir.path().join("empty_dir");
    std::fs::create_dir(&test_dir).unwrap();

    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "empty_dir"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "remove_dir",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_ok());
    assert!(!test_dir.exists());
}

#[test]
fn test_remove_dir_non_empty() {
    let temp_dir = TempDir::new().unwrap();
    let test_dir = temp_dir.path().join("non_empty_dir");
    std::fs::create_dir_all(&test_dir).unwrap();
    std::fs::write(test_dir.join("file.txt"), b"content").unwrap();
    std::fs::create_dir(test_dir.join("subdir")).unwrap();

    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "non_empty_dir"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "remove_dir",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_ok());
    assert!(!test_dir.exists());
}

#[test]
fn test_remove_dir_nested() {
    let temp_dir = TempDir::new().unwrap();
    let nested_dir = temp_dir.path().join("level1").join("level2");
    std::fs::create_dir_all(&nested_dir).unwrap();

    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "level1/level2"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "remove_dir",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_ok());
    assert!(!nested_dir.exists());
    // level1 should still exist
    assert!(temp_dir.path().join("level1").exists());
}

#[test]
fn test_remove_dir_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "nonexistent"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "remove_dir",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_err());
}
