use tracing::error;

use crate::components::stats::PingStats;
use crate::ping::Ping;

use std::fs::{self, read_dir};
use std::path::Path;

pub fn accumulate(pings: &[Ping], time: i64) -> PingStats {
    let mut stats = pings.into_iter().fold(
        PingStats {
            time,
            min: f64::MAX,
            ..PingStats::default()
        },
        |mut stats, ping| {
            if ping.ping < 1000.0 {
                stats.avg += ping.ping;
                stats.max = stats.max.max(ping.ping);
                stats.min = stats.min.min(ping.ping);
            }
            stats.lost += (ping.ping > 1000.0) as usize;
            stats.count += 1;
            stats
        },
    );
    stats.avg /= stats.count as f64;
    stats
}

/// Returns the filenames of the log files in alphabetical order
pub fn log_files(dir: &Path) -> Vec<String> {
    if let Ok(files) = read_dir(dir) {
        let mut files: Vec<_> = files
            .filter_map(|s| s.map(|s| s.file_name().to_string_lossy().into_owned()).ok())
            .filter(|name| is_log_file(name))
            .collect();
        files.sort_unstable();
        files
    } else {
        Vec::new()
    }
}

/// Parses the log files and returns the pings for the given range
/// as the output is reversed and begins with the newest timestamp,
/// `start` has to be larger (after) than `end`.
pub fn read_log<P: AsRef<Path>>(
    log_dir: P,
    offset: usize,
    count: usize,
    start: i64,
    end: i64,
) -> Vec<Ping> {
    assert!(start >= end);

    read_log_all(log_dir.as_ref())
        .skip_while(|ping| start != 0 && ping.time >= start)
        .skip(offset)
        .take(count)
        .take_while(|ping| end == 0 || ping.time >= end)
        .collect()
}

/// Returns an iterator over the past pings from the log files
fn read_log_all(log_dir: &Path) -> impl Iterator<Item = Ping> {
    let log_dir_buf = log_dir.to_owned();
    let mut files = log_files(log_dir);
    files.reverse();
    files
        .into_iter()
        .flat_map(move |file| read_log_file(&log_dir_buf, Path::new(&file)).into_iter())
}

/// Parses the logfile and returns the pings
fn read_log_file(log_dir: &Path, file: &Path) -> Vec<Ping> {
    let filename = log_dir.join(file);
    if let Ok(input) = fs::read_to_string(&filename) {
        parse(&input)
    } else {
        error!("Error opening file {:?}\n", filename);
        Vec::new()
    }
}

fn parse(input: &str) -> Vec<Ping> {
    let mut result = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Ping>, ()>>()
        .unwrap_or_default();
    result.reverse();
    result
}

fn is_log_file(name: &str) -> bool {
    name.len() == 10 && name.ends_with(".txt") && name[0..6].chars().all(char::is_numeric)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("1626457680 11.5\n1626457740 1000\n1626462480 13.9\n"),
            vec![
                Ping::new(1626462480, 13.9),
                Ping::new(1626457740, 1000.0),
                Ping::new(1626457680, 11.5),
            ]
        );
    }
}
