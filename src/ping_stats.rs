extern crate chrono;
extern crate serde_json;

use self::chrono::{Duration, Timelike};
use self::serde_json::*;

use super::ping::Ping;

use std::f64::NAN;
use std::fs::{read_dir, write, File};
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub fn generate_statistics(log_dir: &String, web_dir: &String) {
    let data = generate_json(log_dir);
    match write(web_dir.clone() + "/data.json", data) {
        Err(e) => {
            eprintln!("fs data error: {}", e);
        }
        _ => println!("Update data."),
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

fn generate_json(log_dir: &String) -> String {
    let files = log_files(log_dir);
    let log = read_log(&files).unwrap_or(vec![]);

    let data = json!({
        "log":  generate_log(&log),
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
            let time = values[0].parse::<i64>();
            let ping = values[1].parse::<f64>();

            if time.is_ok() && ping.is_ok() {
                log.push(Ping::new(time.unwrap(), ping.unwrap()));
            }
        }
    }
    log.reverse();
    Ok(log)
}

fn generate_log(log: &Vec<Ping>) -> Vec<(String, f64)> {
    log.into_iter()
        .take(60)
        .map(Ping::tuple)
        .collect::<Vec<_>>()
}

fn generate_history(log: &[Ping]) -> Vec<(String, f64, f64, f64, String)> {
    let mut chunks: Vec<(String, f64, f64, f64, String)> = vec![];
    if log.len() > 0 {
        let mut start = 0;
        let mut end = 0;
        let mut until = log[0].date_time();
        until = until - Duration::minutes(until.minute().into());
        until = until - Duration::seconds(until.second().into());
        until = until - Duration::nanoseconds(until.nanosecond().into());

        for l in log {
            if l.date_time() < until {
                let time_range = until.format("%d.%m.%y %H").to_string()
                    + (until + chrono::Duration::hours(1))
                        .format(" - %Hh")
                        .to_string()
                        .as_str();
                let stats = generate_stats(&log[start..end]);
                chunks.push((time_range, stats.0, stats.1, stats.2, stats.3));

                start = end;
                until = until - chrono::Duration::hours(1);
            } else {
                end += 1;
            }
        }
    }

    chunks
}

fn generate_stats(log: &[Ping]) -> (f64, f64, f64, String) {
    let mut min = 1000.0;
    let mut max = 0.0;
    let mut sum = 0.0;
    let mut lost: i32 = 0;

    for entry in log {
        if entry.latency < min {
            min = entry.latency;
        }
        if entry.latency >= 1000.0 {
            lost += 1;
        } else {
            if entry.latency > max {
                max = entry.latency;
            }
            sum += entry.latency;
        }
    }

    let avg = ((sum * 100.0) / (log.len() - lost as usize) as f64).round() as f64 / 100.0;

    if min >= 1000.0 {
        min = NAN;
    }
    if max <= 0.0 {
        max = NAN;
    }

    (min, max, avg, format!("{} / {}", lost, log.len()))
}
