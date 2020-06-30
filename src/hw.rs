use std::fs;

use serde::Serialize;

/// Path to the sysfs process file with the CPU load average
const LOAD_AVG_FILE: &str = "/proc/loadavg";
/// Path to the sysfs process file with the memory info
const MEM_INFO_FILE: &str = "/proc/meminfo";
/// Path to the HWMON device file with the CPU temperature
const TEMPERATURE_FILE: &str = "/sys/class/thermal/thermal_zone0/temp";

/// Describes the system status of the underlaying linux server.
#[derive(Debug, Clone, Serialize)]
pub struct Status {
    /// CPU load in percent times the number of CPUs.
    load: f32,
    /// Current memory consumption.
    memory_used: f32,
    /// Total memory installed on the system.
    memory_total: f32,
    /// CPU temperature.
    temperature: f32,
}

impl Status {
    /// Load the current system status using Linux's sysfs.
    pub fn request() -> Status {
        let load = request_load();
        let (memory_used, memory_total) = request_mem();
        let temperature = request_temperature();

        Status {
            load,
            memory_used,
            memory_total,
            temperature,
        }
    }
}

/// Returns the current CPU load.
fn request_load() -> f32 {
    if let Ok(load) = fs::read_to_string(LOAD_AVG_FILE) {
        load.splitn(3, ' ')
        .nth(1)
        .and_then(|l| l.parse::<f32>().ok())
        .unwrap_or_default() / num_cpus::get() as f32
    } else {
        0.0
    }
}

/// Returns the currently used and total memory.
fn request_mem() -> (f32, f32) {
    if let Ok(load) = fs::read_to_string(MEM_INFO_FILE) {
        let mut memory_avaliable = 0.0;
        let mut memory_total = 0.0;
        for line in load.split('\n') {
            if line.starts_with("MemTotal:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    memory_total = val.parse::<f32>().unwrap_or_default() / (1024.0 * 1024.0);
                    if memory_avaliable > 0.0 {
                        break;
                    }
                }
            } else if line.starts_with("MemAvailable:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    memory_avaliable = val.parse::<f32>().unwrap_or_default() / (1024.0 * 1024.0);
                    if memory_total > 0.0 {
                        break;
                    }
                }
            }
        }
        (memory_total - memory_avaliable, memory_total)
    } else {
        (0.0, 0.0)
    }
}

/// Returns the currently CPU temperature.
fn request_temperature() -> f32 {
    if let Ok(temperature) = fs::read_to_string(TEMPERATURE_FILE) {
        temperature.trim().parse::<f32>().unwrap_or_default() / 1000.0
    } else {
        0.0
    }
}
