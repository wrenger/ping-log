use std::fs;

use serde::Serialize;

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
        let mut sysinfo = unsafe { std::mem::zeroed() };
        let (load, memory_used, memory_total) = if unsafe { libc::sysinfo(&mut sysinfo) } == 0 {
            let cores = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };
            (
                sysinfo.loads[1] as f32 / (65536.0 / 100.0 * cores as f32),
                ((sysinfo.totalram - sysinfo.freeram) >> 20) as f32 / 1024.0,
                (sysinfo.totalram >> 20) as f32 / 1024.0,
            )
        } else {
            eprintln!("Error reading sysinfo");
            (0.0, 0.0, 0.0)
        };

        let temperature = request_temperature();

        Status {
            load,
            memory_used,
            memory_total,
            temperature,
        }
    }
}

/// Returns the currently CPU temperature.
fn request_temperature() -> f32 {
    fs::read_to_string(TEMPERATURE_FILE)
        .map(|v| v.trim().parse::<f32>().unwrap_or_default() / 1000.0)
        .unwrap_or_default()
}
