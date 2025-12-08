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
fn test_remove_file_existing() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    std::fs::write(&test_file, b"content").unwrap();

    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "test.txt"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "remove_file",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_ok());
    assert!(!test_file.exists());
}

#[test]
fn test_remove_file_nested() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::create_dir_all(temp_dir.path().join("nested")).unwrap();
    let test_file = temp_dir.path().join("nested").join("file.txt");
    std::fs::write(&test_file, b"content").unwrap();

    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "nested/file.txt"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "remove_file",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_ok());
    assert!(!test_file.exists());
}

#[test]
fn test_remove_file_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "nonexistent.txt"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "remove_file",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_err());
}
