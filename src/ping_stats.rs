use super::ping::{History, Ping};

use std::fs::{self, read_dir};
use std::path::Path;

fn is_log_file(name: &str) -> bool {
    name.len() == 10 && name.ends_with(".txt") && name[0..6].chars().all(char::is_numeric)
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
        vec![]
    }
}

/// Parses the log files and returns the pings for the given range
/// As the output is reversed and begins with the newest timestamp,
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
        eprintln!("Error opening file {:?}\n", &filename);
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

/// Returns the accumulated pings per hour for the given period
pub fn read_history<P: AsRef<Path>>(
    log_dir: P,
    offset: usize,
    count: usize,
    start: i64,
    end: i64,
) -> Vec<History> {
    let pings = read_log(log_dir, offset, count * 65, start, end);
    generate_history(&pings)
}

/// Generates a list of accumulated pings per hour
fn generate_history(log: &[Ping]) -> Vec<History> {
    let mut chunks: Vec<History> = vec![];
    let mut start = 0;
    let mut end = 0;
    let mut until = if !log.is_empty() {
        log[0].time / 3600 * 3600
    } else {
        0
    };

    for l in log {
        while l.time < until {
            chunks.push(History::from(until + 3600, &log[start..end]));

            start = end;
            until -= 3600;
        }
        end += 1;
    }
    if end > start {
        chunks.push(History::from(until + 3600, &log[start..end]));
    }

    chunks
}

#[cfg(test)]
mod test {
    use super::*;

    use std::f64::NAN;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("1626457680 11.5\n1626457740 1000\n1626462480 13.9\n"),
            vec![
                Ping::new(1626457680, 11.5),
                Ping::new(1626457740, 1000.0),
                Ping::new(1626462480, 13.9),
            ]
        );
    }

    #[test]
    fn test_generate_history() {
        let log = [Ping::new(1536062893, 10.0), Ping::new(1536059293, 20.0)];

        let history = generate_history(&log);

        assert_eq!(2, history.len());
        assert_eq!(
            History::new(1536066000, 10.0, 10.0, 10.0, 0.0, 1),
            history[0]
        );
        assert_eq!(
            History::new(1536062400, 20.0, 20.0, 20.0, 0.0, 1),
            history[1]
        );

        let log = [Ping::new(1536062893, 10.0), Ping::new(1536055693, 20.0)];

        let history = generate_history(&log);
        println!("{:?}", history);

        assert_eq!(3, history.len());
        assert_eq!(
            History::new(1536066000, 10.0, 10.0, 10.0, 0.0, 1),
            history[0]
        );
        assert_eq!(History::new(1536062400, NAN, NAN, NAN, NAN, 0), history[1]);
        assert_eq!(
            History::new(1536058800, 20.0, 20.0, 20.0, 0.0, 1),
            history[2]
        );
    }
}
