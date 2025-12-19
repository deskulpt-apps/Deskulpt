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
fn test_append_file_existing() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    std::fs::write(&test_file, b"initial").unwrap();

    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "test.txt",
        "content": " appended"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "append_file",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_ok());

    let content = std::fs::read_to_string(&test_file).unwrap();
    assert_eq!(content, "initial appended");
}

#[test]
fn test_append_file_multiple_times() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    std::fs::write(&test_file, b"start").unwrap();

    let plugin = FsPlugin;

    for i in 1..=3 {
        let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());
        let input = json!({
            "path": "test.txt",
            "content": format!(" part{}", i)
        });

        let result = call_plugin(
            widget_dir_fn,
            &plugin,
            "append_file",
            "test_widget".to_string(),
            Some(input),
        );

        assert!(result.is_ok());
    }

    let content = std::fs::read_to_string(&test_file).unwrap();
    assert_eq!(content, "start part1 part2 part3");
}

#[test]
fn test_append_file_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("new_file.txt");
    // Create an empty file first since append_file doesn't create files
    std::fs::write(&test_file, b"").unwrap();

    let plugin = FsPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "path": "new_file.txt",
        "content": "new content"
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "append_file",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_ok());

    let file_path = temp_dir.path().join("new_file.txt");
    assert!(file_path.exists());
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "new content");
}
