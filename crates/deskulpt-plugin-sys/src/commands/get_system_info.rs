use anyhow::{Ok, Result};
use deskulpt_plugin::{EngineInterface, PluginCommand, dispatch};
use serde::Serialize;
use sysinfo::{Disks, Networks, System};

use crate::SysPlugin;

pub struct GetSystemInfo;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuInfo {
    vendor_id: String,
    brand: String,
    frequency: u64,
    total_cpu_usage: f32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskInfo {
    name: String,
    available_space: u64,
    total_space: u64,
    mount_point: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInfo {
    interface_name: String,
    total_received: u64,
    total_transmitted: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSystemInfoOutputPayload {
    total_swap: u64,
    used_swap: u64,
    system_name: Option<String>,
    kernel_version: Option<String>,
    os_version: Option<String>,
    host_name: Option<String>,
    cpu_count: usize,
    cpu_info: Vec<CpuInfo>,
    disks: Vec<DiskInfo>,
    networks: Vec<NetworkInfo>,
    total_memory: u64,
    used_memory: u64,
}

impl PluginCommand for GetSystemInfo {
    type Plugin = SysPlugin;

    fn name(&self) -> &str {
        "get_system_info"
    }

    #[dispatch]
    fn run(
        &self,
        _id: String,
        plugin: &Self::Plugin,
        _engine: &EngineInterface,
        input: (),
    ) -> Result<GetSystemInfoOutputPayload> {
        let mut sys = plugin.0.lock().unwrap();
        sys.refresh_all();

        let disks_info: Vec<DiskInfo> = Disks::new_with_refreshed_list()
            .iter()
            .map(|disk| DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                available_space: disk.available_space(),
                total_space: disk.total_space(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
            })
            .collect();

        let networks_info: Vec<NetworkInfo> = Networks::new_with_refreshed_list()
            .iter()
            .map(|(interface_name, data)| NetworkInfo {
                interface_name: interface_name.to_string(),
                total_received: data.total_received(),
                total_transmitted: data.total_transmitted(),
            })
            .collect();

        let cpu_count = sys.cpus().len();
        let mut cpu_info = Vec::with_capacity(cpu_count);
        for cpu in sys.cpus() {
            cpu_info.push(CpuInfo {
                vendor_id: cpu.vendor_id().to_string(),
                brand: cpu.brand().to_string(),
                frequency: cpu.frequency(),
                total_cpu_usage: cpu.cpu_usage(),
            });
        }

        Ok(GetSystemInfoOutputPayload {
            total_memory: sys.total_memory(),
            used_memory: sys.used_memory(),
            total_swap: sys.total_swap(),
            used_swap: sys.used_swap(),
            system_name: System::name(),
            kernel_version: System::kernel_version(),
            os_version: System::os_version(),
            host_name: System::host_name(),
            cpu_count,
            cpu_info,
            disks: disks_info,
            networks: networks_info,
        })
    }
}
