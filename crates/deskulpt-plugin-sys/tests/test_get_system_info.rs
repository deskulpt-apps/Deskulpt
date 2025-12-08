use std::path::PathBuf;
use std::sync::Mutex;

use anyhow::Result;
use deskulpt_plugin::call_plugin;
use deskulpt_plugin_sys::SysPlugin;
use serde_json::json;
use sysinfo::System;

fn create_widget_dir_fn() -> impl Fn(&str) -> Result<PathBuf> + 'static {
    move |_id: &str| -> Result<PathBuf> { Ok(PathBuf::from("/tmp")) }
}

#[test]
fn test_get_system_info_returns_valid_structure() {
    let plugin = SysPlugin(Mutex::new(System::new()));
    let widget_dir_fn = create_widget_dir_fn();

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "get_system_info",
        "test_widget".to_string(),
        Some(json!(null)),
    );

    assert!(result.is_ok());
    let info: serde_json::Value = result.unwrap();

    // Verify all required fields are present
    assert!(info.get("totalSwap").is_some());
    assert!(info.get("usedSwap").is_some());
    assert!(info.get("systemName").is_some());
    assert!(info.get("kernelVersion").is_some());
    assert!(info.get("osVersion").is_some());
    assert!(info.get("hostName").is_some());
    assert!(info.get("cpuCount").is_some());
    assert!(info.get("cpuInfo").is_some());
    assert!(info.get("disks").is_some());
    assert!(info.get("networks").is_some());
    assert!(info.get("totalMemory").is_some());
    assert!(info.get("usedMemory").is_some());

    // Verify cpuInfo is an array
    let cpu_info = info.get("cpuInfo").unwrap().as_array().unwrap();
    assert_eq!(
        cpu_info.len(),
        info.get("cpuCount").unwrap().as_u64().unwrap() as usize
    );

    // Verify disks is an array
    assert!(info.get("disks").unwrap().as_array().is_some());

    // Verify networks is an array
    assert!(info.get("networks").unwrap().as_array().is_some());
}

#[test]
fn test_get_system_info_no_panic() {
    let plugin = SysPlugin(Mutex::new(System::new()));

    // Call multiple times to ensure no panics
    for _ in 0..5 {
        let result = call_plugin(
            create_widget_dir_fn(),
            &plugin,
            "get_system_info",
            "test_widget".to_string(),
            Some(json!(null)),
        );
        assert!(result.is_ok());
    }
}

#[test]
fn test_get_system_info_cpu_info_structure() {
    let plugin = SysPlugin(Mutex::new(System::new()));
    let widget_dir_fn = create_widget_dir_fn();

    let result = call_plugin(
        widget_dir_fn,
        &plugin,
        "get_system_info",
        "test_widget".to_string(),
        Some(json!(null)),
    );

    assert!(result.is_ok());
    let info: serde_json::Value = result.unwrap();
    let cpu_info = info.get("cpuInfo").unwrap().as_array().unwrap();

    if !cpu_info.is_empty() {
        let first_cpu = &cpu_info[0];
        assert!(first_cpu.get("vendorId").is_some());
        assert!(first_cpu.get("brand").is_some());
        assert!(first_cpu.get("frequency").is_some());
        assert!(first_cpu.get("totalCpuUsage").is_some());
    }
}
