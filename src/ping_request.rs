extern crate chrono;

use self::chrono::prelude::*;

use std::path::Path;
use std::fs::{OpenOptions};
use std::io::{Result, Write};

use std::process::Command;

pub fn ping_request(host: &String, log_dir: &String) {
    let log = perform_request(host);
    match write_request(log_dir.clone() + "/180827.txt", log) {
        Err(e) => { eprintln!("fs log error: {}", e) },
        _ => { println!("Update log.") }
    }
}


fn perform_request(host: &String) -> (i64, f64) {
    let time = Local::now().timestamp();
    let output = Command::new("ping")
            .args(&["-c 1", "-w 1", host])
            .output()
            .expect("failed to execute ping");

    if !output.stderr.is_empty() {
        eprintln!("Command error: {}", String::from_utf8_lossy(&output.stderr));
    }

    let ping = parse_output(String::from_utf8(output.stdout).unwrap_or("".to_string()));
    (time, ping)
}

fn parse_output(output: String) -> f64 {
    let lines = output.splitn(3, '\n').collect::<Vec<_>>();
    if lines.len() >= 3 {
        let line = lines[1];
        let start = line.rfind('=');
        let end = line.rfind(' ');
        if start.is_some() && end.is_some() {
            let ping_str = &line[start.unwrap() + 1..end.unwrap()];
            return ping_str.parse::<f64>().unwrap_or(1000.0)
        }
    }
    1000.0
}

fn write_request<P: AsRef<Path>>(path: P, log: (i64, f64)) -> Result<()> {
    let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path)?;
    file.write_fmt(format_args!("{} {}\n", log.0, log.1))?;
    Ok(())
}
