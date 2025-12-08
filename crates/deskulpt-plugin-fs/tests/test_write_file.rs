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
fn test_write_file_new() {
    let temp_dir = TempDir::new().unwrap();
    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "new_file.txt",
        "content": "new content"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "write_file",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_ok());

    let file_path = temp_dir.path().join("new_file.txt");
    assert!(file_path.exists());
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "new content");
}

#[test]
fn test_write_file_overwrite() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("existing.txt");
    std::fs::write(&test_file, b"old content").unwrap();

    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "existing.txt",
        "content": "new content"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "write_file",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_ok());

    let content = std::fs::read_to_string(&test_file).unwrap();
    assert_eq!(content, "new content");
}

#[test]
fn test_write_file_nested_path() {
    let temp_dir = TempDir::new().unwrap();
    // Create parent directories since write_file doesn't create them
    let nested_dir = temp_dir.path().join("nested").join("dir");
    std::fs::create_dir_all(&nested_dir).unwrap();

    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "nested/dir/file.txt",
        "content": "nested content"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "write_file",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_ok());

    let file_path = temp_dir.path().join("nested").join("dir").join("file.txt");
    assert!(file_path.exists());
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "nested content");
}
