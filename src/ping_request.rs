use std::fs::OpenOptions;
use std::fs::{read_dir, remove_file};
use std::io::{Result, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use chrono::{Duration, Local};

use super::ping::Ping;

/// Performs an ping request and stores the result in the log file
pub fn ping_request(host: &String, log_dir: &PathBuf) {
    let log = perform_request(host);
    let file_name = Local::today().format("%y%m%d.txt").to_string();

    write_request(log_dir, file_name, log).expect("write log error");
}

fn perform_request(host: &String) -> Ping {
    let time = Local::now().timestamp();
    let output = Command::new("ping")
        .args(&["-c 1", "-w 1", &host])
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

fn write_request<P: AsRef<Path>, Q: AsRef<Path>>(dir: P, filename: Q, log: Ping) -> Result<()> {
    let path = Path::new(dir.as_ref()).join(filename);

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

fn remove_old_logs<P: AsRef<Path>>(dir: P) {
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
fn older(filename: &String, oldest: &String) -> bool {
    filename.len() == 10 && filename.ends_with(".txt") && filename < oldest
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_old_filename() {
        assert_eq!(10, "191129.txt".len());

        assert!(!older(&"malformed".into(), &"191129".into()));

        assert!(older(&"191029.txt".into(), &"191129".into()));
        assert!(older(&"191129.txt".into(), &"191130".into()));

        assert!(!older(&"191129.txt".into(), &"191129".into()));
        assert!(!older(&"191129.txt".into(), &"191128".into()));
    }
}
