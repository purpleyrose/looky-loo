use sysinfo::{RefreshKind, System, ProcessRefreshKind, MINIMUM_CPU_UPDATE_INTERVAL};
use crate::data::{system_data::SystemStats, process_data::ProcessData};
use std::time::Duration;
use std::thread;


pub fn get_system_stats(sys: &mut System) -> SystemStats {
    // Refresh system information
    sys.refresh_memory();
    sys.refresh_cpu_all();

    // Retrieve system-wide CPU and memory usage
    SystemStats {
        cpu_usage: sys.global_cpu_usage(),
        memory_usage: sys.used_memory(),
        memory_total: sys.total_memory(),
    }
}
pub fn get_process_list(sys: &mut System) -> Vec<ProcessData> {
    // Refresh processes with all available information
    let refresh_kind = RefreshKind::new().with_processes(ProcessRefreshKind::everything());
    
    // First refresh
    sys.refresh_specifics(refresh_kind);

    // Wait for the minimum required time for accurate CPU calculation
    thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);  // Use the minimum interval specified by sysinfo

    // Second refresh to calculate CPU usage properly
    sys.refresh_specifics(refresh_kind);

    // Retrieve process data with the updated CPU usage
    sys.processes()
        .values()
        .map(|p| ProcessData {
            pid: p.pid().as_u32() as i32,
            name: p.name().to_string_lossy().into_owned(),
            cpu_usage: p.cpu_usage(),  // This value is calculated over time
            memory: p.memory(),  // This is reported in kilobytes
        })
        .collect()
}
