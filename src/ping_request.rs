use std::fs::OpenOptions;
use std::fs::{read_dir, remove_file};
use std::io::{Result, Write};
use std::path::Path;
use std::process::Command;

use chrono::{Duration, Local};

use super::ping::Ping;

#[cfg(not(target_os = "macos"))]
const WAIT_ARG: &str = "-w 1";
#[cfg(target_os = "macos")]
const WAIT_ARG: &str = "-W 1";

const COUNT_ARG: &str = "-c 1";

/// Performs an ping request and stores the result in the log file
pub fn ping_request(host: &str, log_dir: &Path) {
    let log = perform_request(host);
    let file_name = Local::today().format("%y%m%d.txt").to_string();

    write_request(log_dir, Path::new(&file_name), log).expect("write log error");
}

fn perform_request(host: &str) -> Ping {
    let time = Local::now().timestamp();
    let output = Command::new("ping")
        .args(&[COUNT_ARG, WAIT_ARG, &host])
        .output()
        .expect("failed to execute bash 'ping' command");

    if !output.stderr.is_empty() {
        eprintln!(
            "bash 'ping' error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ping::new(
        time,
        String::from_utf8(output.stdout).map_or(1000.0, |o| parse_output(&o)),
    )
}

fn parse_output(output: &str) -> f64 {
    if let Some(line) = output.splitn(3, '\n').nth(1) {
        if let Some(start) = line.rfind('=') {
            if let Some(end) = line.rfind(' ') {
                return line[start + 1..end].parse().unwrap_or(1000.0);
            }
        }
    }
    1000.0
}

fn write_request(dir: &Path, filename: &Path, log: Ping) -> Result<()> {
    if !dir.exists() {
        std::fs::create_dir_all(dir).expect("Error creating log dir");
    }

    let path = dir.join(filename);

    if !path.exists() {
        remove_old_logs(dir);
    }

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)?;
    file.write_fmt(format_args!("{} {}\n", log.time, log.ping))?;
    Ok(())
}

fn remove_old_logs(dir: &Path) {
    let oldest = Local::now() - Duration::weeks(8);
    let oldest = oldest.format("%y%m%d").to_string();

    if let Ok(entries) = read_dir(dir) {
        entries.for_each(|entry| {
            if let Ok(entry) = entry {
                let filename = entry.file_name().to_string_lossy().into_owned();
                if older(&filename, &oldest) {
                    remove_file(entry.path()).expect("Could not remove old log");
                }
            }
        })
    }
}

/// Is the given `filename` older than the formatted date `oldest`
fn older(filename: &str, oldest: &str) -> bool {
    filename.len() == 10 && filename.ends_with(".txt") && filename < oldest
}

#[cfg(test)]
mod test {

    #[test]
    fn parse_output() {
        use super::*;

        assert_eq!(
            parse_output(
                "\
PING 1.1.1.1 (1.1.1.1) 56(84) bytes of data.
64 bytes from 1.1.1.1: icmp_seq=1 ttl=57 time=11.3 ms

--- 1.1.1.1 ping statistics ---
1 packets transmitted, 1 received, 0% packet loss, time 0ms
rtt min/avg/max/mdev = 11.315/11.315/11.315/0.000 ms"
            ),
            11.3
        )
    }

    #[test]
    fn old_filename() {
        use super::*;

        assert_eq!(10, "191129.txt".len());

        assert!(!older("malformed", "191129"));

        assert!(older("191029.txt", "191129"));
        assert!(older("191129.txt", "191130"));

        assert!(!older("191129.txt", "191129"));
        assert!(!older("191129.txt", "191128"));
    }

    #[test]
    fn time() {
        use chrono::{Local, Utc};
        println!("Current time: {}", Local::now());
        println!("Current time: {}", Local::now().timestamp());
        println!("UTC time: {}", Utc::now());
        println!("UTC time: {}", Utc::now().timestamp());
    }
}
