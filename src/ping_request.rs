use chrono::Local;

use super::ping::Ping;

use std::fs::OpenOptions;
use std::io::{Result, Write};
use std::path::Path;

use std::process::Command;

pub fn ping_request(host: &String, log_dir: &String) {
    let log = perform_request(host);

    let file_name = Local::now().format("/%y%m%d.txt").to_string();

    match write_request(log_dir.clone() + file_name.as_str(), log) {
        Err(e) => eprintln!("write log error: {}", e),
        _ => println!("update log."),
    }
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

    let ping = parse_output(String::from_utf8(output.stdout).unwrap_or_default());
    Ping::new(time, ping)
}

fn parse_output(output: String) -> f64 {
    let lines = output.splitn(3, '\n').collect::<Vec<_>>();
    if lines.len() >= 3 {
        let line = lines[1];
        let start = line.rfind('=');
        let end = line.rfind(' ');
        if start.is_some() && end.is_some() {
            let ping_str = &line[start.unwrap() + 1..end.unwrap()];
            return ping_str.parse().unwrap_or(1000.0);
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
