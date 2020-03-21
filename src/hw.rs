use std::fs;

use serde::Serialize;

const LOAD_AVG_FILE: &str = "/proc/loadavg";
const CPU_TEMPERATURE: &str = "/sys/class/thermal/thermal_zone0/temp";

#[derive(Debug, Clone, Serialize)]
pub struct Status {
    load: f32,
    temperature: f32,
}

impl Status {
    pub fn request() -> Status {
        let load = if let Ok(load) = fs::read_to_string(LOAD_AVG_FILE) {
            load.splitn(3, ' ')
                .nth(1)
                .and_then(|l| l.parse::<f32>().ok())
                .unwrap_or_default()
        } else {
            0.0
        };

        let temperature = if let Ok(temperature) = fs::read_to_string(CPU_TEMPERATURE) {
            temperature.trim().parse::<f32>().unwrap_or_default() / 1000.0
        } else {
            0.0
        };

        Status { load, temperature }
    }
}
