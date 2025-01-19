use std::fs;

use tracing::error;
use crate::components::hw::HardwareProps;

/// Path to the HWMON device file with the CPU temperature
const TEMPERATURE_FILE: &str = "/sys/class/thermal/thermal_zone0/temp";

pub fn request() -> HardwareProps {
    let mut sysinfo = unsafe { std::mem::zeroed() };
    let (load, memory_used, memory_total) = if unsafe { libc::sysinfo(&mut sysinfo) } == 0 {
        let cores = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };
        (
            sysinfo.loads[1] as f32 / (65536.0 / 100.0 * cores as f32),
            ((sysinfo.totalram - sysinfo.freeram) >> 20) as f32 / 1024.0,
            (sysinfo.totalram >> 20) as f32 / 1024.0,
        )
    } else {
        error!("Error reading sysinfo");
        (0.0, 0.0, 0.0)
    };

    let temperature = request_temperature();

    HardwareProps {
        load,
        memory_used,
        memory_total,
        temperature,
    }
}

/// Returns the currently CPU temperature.
fn request_temperature() -> f32 {
    fs::read_to_string(TEMPERATURE_FILE)
        .map(|v| v.trim().parse::<f32>().unwrap_or_default() / 1000.0)
        .unwrap_or_default()
}
