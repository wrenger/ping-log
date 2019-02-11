use chrono::{Duration, Timelike};

use super::ping::{History, Ping};

use std::f64::NAN;

use std::fs::{read_dir, write, File};
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub fn generate_statistics(log_dir: &String, web_dir: &String) {
    let files = log_files(log_dir);
    let log = read_log(&files).unwrap_or(vec![]);

    let data = generate_json(&log, &files);
    match write(web_dir.clone() + "/data.json", data) {
        Err(e) => {
            eprintln!("write data error: {}", e);
        }
        _ => println!("update data."),
    };
}

pub fn log_files<P: AsRef<Path>>(dir: P) -> Vec<String> {
    let mut files = read_dir(dir)
        .map(|f| {
            f.filter_map(|s| s.ok().map(|s| s.path().to_string_lossy().into_owned()))
                .collect::<Vec<_>>()
        })
        .unwrap_or(vec![]);
    files.sort();
    files
}

fn generate_json(log: &Vec<Ping>, files: &[String]) -> String {
    let data = json!({
        "log": &log[..60.min(log.len())],
        "history": generate_history(&log),
        "files": files
    });
    data.to_string()
}

fn read_log(files: &[String]) -> Result<Vec<Ping>> {
    let mut log = Vec::with_capacity(3 * 60 * 24);
    let max = 1 + 3.min(files.len());
    for i in 1..max {
        let mut daily_log = read_log_file(&files[files.len() - i])?;
        log.append(&mut daily_log);
    }
    Ok(log)
}

fn read_log_file(file: &str) -> Result<Vec<Ping>> {
    let mut log = Vec::with_capacity(60 * 24);
    let file = File::open(&file)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let values = line.splitn(2, ' ').collect::<Vec<_>>();
        if values.len() == 2 {
            if let Ok(time) = values[0].parse::<i64>() {
                if let Ok(ping) = values[1].parse::<f64>() {
                    log.push(Ping::new(time, ping));
                }
            }
        }
    }
    log.reverse();
    Ok(log)
}

fn generate_history(log: &[Ping]) -> Vec<History> {
    let max_count = 3 * 24;

    let mut chunks: Vec<History> = vec![];
    if log.len() > 0 {
        let mut start = 0;
        let mut end = 0;
        let mut until = log[0].date_time();
        until = until - Duration::minutes(until.minute().into());
        until = until - Duration::seconds(until.second().into());
        until = until - Duration::nanoseconds(until.nanosecond().into());
        let mut until_ts = until.timestamp();

        for l in log {
            let mut i = 0;
            while l.time < until_ts && i < max_count {
                chunks.push(History::from((&log[start..end], until.timestamp())));

                start = end;
                until = until - Duration::hours(1);
                until_ts = until.timestamp();
                i += 1;
            }
            if i >= max_count {
                break;
            }
            end += 1;
        }
        if end > start {
            chunks.push(History::from((&log[start..end], until.timestamp())));
        }
    }

    chunks
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_history() {
        let log = [Ping::new(1536062893, 10.0), Ping::new(1536059293, 20.0)];

        let history = generate_history(&log);

        assert_eq!(2, history.len());
        assert_eq!(History::new(1536062400, 10.0, 10.0, 10.0, 0, 1), history[0]);
        assert_eq!(History::new(1536058800, 20.0, 20.0, 20.0, 0, 1), history[1]);

        let log = [Ping::new(1536062893, 10.0), Ping::new(1536055693, 20.0)];

        let history = generate_history(&log);
        println!("{:?}", history);

        assert_eq!(3, history.len());
        assert_eq!(History::new(1536062400, 10.0, 10.0, 10.0, 0, 1), history[0]);
        assert_eq!(History::new(1536058800, NAN, NAN, NAN, 0, 0), history[1]);
        assert_eq!(History::new(1536055200, 20.0, 20.0, 20.0, 0, 1), history[2]);
    }
}
