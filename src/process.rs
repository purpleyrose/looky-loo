use std::{cell::Ref, thread};

use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use crate::data::{process_data::ProcessData, system_data::SystemStats};

pub fn get_system_stats(sys: &mut System) -> SystemStats {
    // Refresh system-wide CPU and memory usage
    sys.refresh_memory();
    sys.refresh_cpu_usage();
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();
    SystemStats {
        cpu_usage: sys.global_cpu_usage(),
        memory_usage: sys.used_memory(),
        memory_total: sys.total_memory(),
    }
}

pub fn get_process_list(sys: &mut System) -> Vec<ProcessData> {
    // Refresh all processes with CPU data
    let refresh_kind = RefreshKind::new().with_processes(ProcessRefreshKind::everything());
    sys.refresh_specifics(refresh_kind);
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_specifics(refresh_kind);
    
    // Collect and return process data
    sys.processes()
        .values()
        .map(|p| ProcessData {
            pid: p.pid().as_u32() as i32,
            name: p.name().to_string_lossy().into_owned(),
            cpu_usage: p.cpu_usage(),  // CPU usage after refresh
            memory: p.memory(),        // Memory usage in kilobytes
        })
        .collect()
}
