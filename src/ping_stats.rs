extern crate serde_json;
extern crate chrono;

use self::serde_json::*;
use self::chrono::prelude::*;

use std::path::Path;
use std::fs::{File, read_dir, write};
use std::io::{Result, BufRead, BufReader};


pub fn generate_statistics(log_dir: &String, web_dir: &String) {
    let data = generate_json(log_dir);
    match write(web_dir.clone() + "/data.json", data) {
        Err(e) => { eprintln!("fs data error: {}", e); }
        _ => { println!("Update data.") }
    };
}

/**
 * Generate the log stats
 */

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

fn read_log_file(file: &str) -> Result<Vec<(i64, f64)>> {
    let mut log = Vec::with_capacity(60 * 24);
    let file = File::open(&file)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let values = line.splitn(2, ' ').collect::<Vec<_>>();
        if values.len() == 2 {
            let time = values[0].parse::<f64>();
            let ping = values[1].parse::<f64>();

            if time.is_ok() && ping.is_ok() {
                log.push((time.unwrap() as i64, ping.unwrap()));
            }
        }
    }
    log.reverse();
    Ok(log)
}


fn read_log(files: &[String]) -> Result<Vec<(i64, f64)>> {
    let mut log = Vec::with_capacity(3 * 60 * 24);
    let max = 1 + 3.min(files.len());
    for i in 1..max {
        let mut daily_log = read_log_file(&files[files.len() - i])?;
        log.append(&mut daily_log);
    }
    Ok(log)
}

fn generate_log(log: &Vec<(i64, f64)>) -> Vec<(String, f64)> {
    log.into_iter().take(60).map(|l| (time_to_string(l.0), l.1)).collect::<Vec<_>>()
}

fn generate_stats(log: &[(i64, f64)]) -> (String, f64, f64, f64, i32) {
    let mut min = 1000.0;
    let mut max = 0.0;
    let mut sum = 0.0;
    let mut lost = 0;

    for entry in log {
        sum += entry.1;

        if entry.1 < min {
            min = entry.1;
        }
        if entry.1 >= 1000.0 {
            lost += 1;
        } else if entry.1 > max {
            max = entry.1;
        }
    }

    let avg = ((sum * 100.0) / log.len() as f64).round() as f64 / 100.0;

    (time_to_string(log[0].0), min, max, avg, lost)
}

fn generate_history(log: &[(i64, f64)]) -> Vec<(String, f64, f64, f64, i32)> {
    log.chunks(60).map(|c| generate_stats(c)).collect::<Vec<_>>()
}

fn log_files<P: AsRef<Path>>(dir: P) -> Vec<String> {
    let mut files = read_dir(dir).map(|f| f.filter_map(|s|
            s.ok().map(|s| s.path().to_string_lossy().into_owned())
        ).collect::<Vec<_>>()).unwrap_or(vec![]);
    files.sort();
    files
}

fn time_to_string(timestamp: i64) -> String {
    let loc = time_from_ts(timestamp);
    loc.format("%d.%m.%y %H:%M").to_string()
}

fn time_from_ts(timestamp: i64) -> DateTime<Local> {
    let native = NaiveDateTime::from_timestamp(timestamp, 0);
    let time: DateTime<Utc> = DateTime::from_utc(native, Utc);

    time.with_timezone(&Local)
}



