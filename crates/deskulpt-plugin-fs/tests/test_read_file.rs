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
fn test_read_file_existing() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    std::fs::write(&test_file, b"hello world").unwrap();

    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "test.txt"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "read_file",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_ok());
    let content: String = serde_json::from_value(result.unwrap()).unwrap();
    assert_eq!(content, "hello world");
}

#[test]
fn test_read_file_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "nonexistent.txt"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "read_file",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_err());
}

#[test]
fn test_read_file_nested_path() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::create_dir_all(temp_dir.path().join("nested")).unwrap();
    let test_file = temp_dir.path().join("nested").join("test.txt");
    std::fs::write(&test_file, b"nested content").unwrap();

    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "nested/test.txt"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "read_file",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_ok());
    let content: String = serde_json::from_value(result.unwrap()).unwrap();
    assert_eq!(content, "nested content");
}
