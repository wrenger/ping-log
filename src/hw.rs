use std::fs;

use serde::Serialize;

const LOAD_AVG_FILE: &str = "/proc/loadavg";
const MEM_INFO_FILE: &str = "/proc/meminfo";
const TEMPERATURE_FILE: &str = "/sys/class/thermal/thermal_zone0/temp";

#[derive(Debug, Clone, Serialize)]
pub struct Status {
    load: f32,
    memory_used: f32,
    memory_total: f32,
    temperature: f32,
}

impl Status {
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

fn request_load() -> f32 {
    if let Ok(load) = fs::read_to_string(LOAD_AVG_FILE) {
        load.splitn(3, ' ')
            .nth(1)
            .and_then(|l| l.parse::<f32>().ok())
            .unwrap_or_default()
    } else {
        0.0
    }
}

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

fn request_temperature() -> f32 {
    if let Ok(temperature) = fs::read_to_string(TEMPERATURE_FILE) {
        temperature.trim().parse::<f32>().unwrap_or_default() / 1000.0
    } else {
        0.0
    }
}
