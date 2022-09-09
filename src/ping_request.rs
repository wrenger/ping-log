use std::fs::OpenOptions;
use std::fs::{read_dir, remove_file};
use std::io::{Result, Write};
use std::path::Path;
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::Local;
use regex::Regex;

use super::ping::Ping;

pub async fn monitor(host: &str, interval: u64, log_dir: &Path) {
    loop {
        let epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let next = Duration::from_secs(((epoch.as_secs() + interval) / interval) * interval);

        tokio::time::sleep(next - epoch).await;

        let log = perform_request(host).await;
        write_request(log_dir, log).expect("write log error");
    }
}

async fn perform_request(host: &str) -> Ping {
    #[cfg(not(target_os = "macos"))]
    const WAIT_ARG: &str = "-w 1";
    #[cfg(target_os = "macos")]
    const WAIT_ARG: &str = "-W 1";

    const COUNT_ARG: &str = "-c 1";

    let time = Local::now().timestamp();
    let output = Command::new("ping")
        .args([COUNT_ARG, WAIT_ARG, host])
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
        String::from_utf8(output.stdout).map_or(1000.0, |o| parse(&o)),
    )
}

fn parse(input: &str) -> f64 {
    lazy_static::lazy_static! {
        static ref PING_RE: Regex =  Regex::new(
            r#"^64 bytes from [\w\.\-:]+( \([\w\.\-:]+\))?: icmp_seq=\d+ ttl=\d+ time=([\d.]+) ms"#,
        )
        .unwrap();
    }

    if let Some((_, input)) = input.split_once('\n') {
        if let Some(found) = PING_RE.captures(input) {
            return found[2].parse().unwrap_or(1000.0);
        }
    }
    1000.0
}

fn write_request(dir: &Path, log: Ping) -> Result<()> {
    if !dir.exists() {
        std::fs::create_dir_all(dir).expect("Error creating log dir");
    }

    let filename = Local::today().format("%y%m%d.txt").to_string();
    let path = dir.join(filename);

    if !path.exists() {
        remove_old_logs(dir);
    }

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{log}")?;
    Ok(())
}

fn remove_old_logs(dir: &Path) {
    let oldest = Local::now() - chrono::Duration::weeks(8);
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
    fn parse() {
        use super::*;

        assert_eq!(
            parse(
                "\
PING 1.1.1.1 (1.1.1.1) 56(84) bytes of data.
64 bytes from 1.1.1.1: icmp_seq=1 ttl=57 time=11.3 ms

--- 1.1.1.1 ping statistics ---
1 packets transmitted, 1 received, 0% packet loss, time 0ms
rtt min/avg/max/mdev = 11.315/11.315/11.315/0.000 ms\n"
            ),
            11.3
        );
        assert_eq!(
            parse("\
PING google.com(fra07s29-in-x200e.1e100.net (2a00:1450:4001:802::200e)) 56 data bytes
64 bytes from fra07s29-in-x200e.1e100.net (2a00:1450:4001:802::200e): icmp_seq=1 ttl=118 time=15.9 ms

--- google.com ping statistics ---
1 packets transmitted, 1 received, 0% packet loss, time 0ms
rtt min/avg/max/mdev = 15.877/15.877/15.877/0.000 ms\n"),
            15.9
        );
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
