use sysinfo::System;

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub memory_mb: u64,
}

#[derive(Clone, Copy)]
pub enum SortKey {
    Cpu,
    Memory,
}

pub fn fetch_processes(sys: &System, limit: usize, sort_by: SortKey) -> Vec<ProcessInfo> {
    let mut processes: Vec<_> = sys
        .processes()
        .iter()
        .map(|(pid, process)| ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().to_string(),
            cpu_percent: process.cpu_usage(),
            memory_mb: process.memory() / 1024 / 1024,
        })
        .collect();

    match sort_by {
        SortKey::Cpu => {
            processes.sort_by(|a, b| {
                b.cpu_percent
                    .partial_cmp(&a.cpu_percent)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }
        SortKey::Memory => {
            processes.sort_by(|a, b| b.memory_mb.cmp(&a.memory_mb));
        }
    }

    processes.into_iter().take(limit).collect()
}
