use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{
    EngineInterface, Plugin, PluginCommand, call_plugin, dispatch, register_commands,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tempfile::TempDir;

// Mock plugin for testing
struct MockPlugin;

impl Plugin for MockPlugin {
    register_commands![MockCommand, MockCommandWithPayload];
}

// Mock command that returns a simple value
struct MockCommand;

#[derive(Serialize)]
struct MockCommandOutput {
    message: String,
}

impl PluginCommand for MockCommand {
    type Plugin = MockPlugin;

    fn name(&self) -> &str {
        "mock_command"
    }

    fn run(
        &self,
        _id: String,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        _input: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let output = MockCommandOutput {
            message: "success".to_string(),
        };
        Ok(serde_json::to_value(output)?)
    }
}

// Mock command that takes and returns payload
struct MockCommandWithPayload;

#[derive(Deserialize)]
struct MockCommandInput {
    value: i32,
}

#[derive(Serialize)]
struct MockCommandPayloadOutput {
    doubled: i32,
}

impl PluginCommand for MockCommandWithPayload {
    type Plugin = MockPlugin;

    fn name(&self) -> &str {
        "mock_command_with_payload"
    }

    #[dispatch]
    fn run(
        &self,
        _id: String,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        input: MockCommandInput,
    ) -> Result<MockCommandPayloadOutput> {
        Ok(MockCommandPayloadOutput {
            doubled: input.value * 2,
        })
    }
}

fn create_widget_dir_fn(temp_path: PathBuf) -> impl Fn(&str) -> Result<PathBuf> + 'static {
    move |_id: &str| -> Result<PathBuf> { Ok(temp_path.clone()) }
}

#[test]
fn test_call_plugin_valid_command() {
    let temp_dir = TempDir::new().unwrap();
    let plugin = MockPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "mock_command",
        "test_widget".to_string(),
        None,
    );

    assert!(result.is_ok());
    let output: serde_json::Value = result.unwrap();
    assert_eq!(output.get("message").unwrap().as_str().unwrap(), "success");
}

#[test]
fn test_call_plugin_with_payload() {
    let temp_dir = TempDir::new().unwrap();
    let plugin = MockPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let input = json!({
        "value": 5
    });

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "mock_command_with_payload",
        "test_widget".to_string(),
        Some(input),
    );

    assert!(result.is_ok());
    let output: serde_json::Value = result.unwrap();
    assert_eq!(output.get("doubled").unwrap().as_i64().unwrap(), 10);
}

#[test]
fn test_call_plugin_invalid_command() {
    let temp_dir = TempDir::new().unwrap();
    let plugin = MockPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "unknown_command",
        "test_widget".to_string(),
        None,
    );

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Unknown command"));
}

#[test]
fn test_call_plugin_null_payload() {
    let temp_dir = TempDir::new().unwrap();
    let plugin = MockPlugin;
    let widget_dir_fn = create_widget_dir_fn(temp_dir.path().to_path_buf());

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "mock_command",
        "test_widget".to_string(),
        Some(json!(null)),
    );

    assert!(result.is_ok());
    let output: serde_json::Value = result.unwrap();
    assert_eq!(output.get("message").unwrap().as_str().unwrap(), "success");
}
