use sysinfo::{ProcessRefreshKind, System};
use crate::data::{system_data::SystemStats, process_data::ProcessData};

pub fn initalize_system() -> System {
    let mut system = System::new_all();
    system.refresh_all();
    system
}

pub fn get_system_stats(sys: &mut System) -> SystemStats {
    sys.refresh_all();  // Refresh system-wide stats like CPU and memory
    SystemStats {
        cpu_usage: sys.global_cpu_usage(),
        memory_usage: sys.used_memory(),
        memory_total: sys.total_memory(),
    }
}

pub fn get_process_list(sys: &mut System) -> Vec<ProcessData> {
    // Refresh process-specific data (like CPU and memory usage)
    sys.refresh_processes_specifics(sysinfo::ProcessesToUpdate::All, ProcessRefreshKind::everything());

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
