use std::fs::OpenOptions;
use std::io::{Result, Write};
use std::path::Path;
use std::process::Command;

use chrono::Local;

use super::ping::Ping;

pub fn ping_request(host: &String, log_dir: &String) {
    let log = perform_request(host);
    let file_name = Local::now().format("%y%m%d.txt").to_string();

    write_request(Path::new(&log_dir).join(file_name), log).expect("write log error: {}");
}

fn perform_request(host: &String) -> Ping {
    let time = Local::now().timestamp();
    let output = Command::new("ping")
        .args(&["-c 1", "-w 1", &host])
        .output()
        .expect("failed to execute bash 'ping' command: {}");

    if !output.stderr.is_empty() {
        eprintln!(
            "bash 'ping' error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ping::new(
        time,
        String::from_utf8(output.stdout)
            .ok()
            .map_or(1000.0, |out| parse_output(out)),
    )
}

fn parse_output(output: String) -> f64 {
    if let Some(line) = output.splitn(3, '\n').nth(1) {
        if let Some(start) = line.rfind('=') {
            if let Some(end) = line.rfind(' ') {
                return line[start + 1..end].parse().unwrap_or(1000.0);
            }
        }
    }
    1000.0
}

fn write_request<P: AsRef<Path>>(path: P, log: Ping) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)?;
    file.write_fmt(format_args!("{} {}\n", log.time, log.ping))?;
    Ok(())
}
