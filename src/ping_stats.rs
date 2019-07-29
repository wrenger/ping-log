use super::ping::{History, Ping};

use std::convert::TryFrom;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};

use std::path::Path;

pub fn log_files<P: AsRef<Path>>(dir: P) -> Vec<String> {
    let mut files: Vec<_> = read_dir(dir)
        .map(|f| {
            f.filter_map(|s| s.map(|s| s.file_name().to_string_lossy().into_owned()).ok())
                .collect()
        })
        .unwrap_or_default();
    files.sort();
    files
}

pub fn read_log<P: AsRef<Path>>(
    log_dir: P,
    offset: usize,
    count: usize,
    start: i64,
    end: i64,
) -> Vec<Ping> {
    read_log_all(log_dir.as_ref())
        .skip_while(move |ping| start != 0 && ping.time > start)
        .skip(offset)
        .take(count)
        .take_while(move |ping| end == 0 || ping.time >= end)
        .collect::<Vec<_>>()
}

fn read_log_all(log_dir: &Path) -> impl Iterator<Item = Ping> {
    let log_dir_buf = log_dir.to_owned();
    log_files(log_dir)
        .into_iter()
        .rev()
        .map(move |file| read_log_file(log_dir_buf.to_owned(), file).into_iter())
        .flatten()
}

fn read_log_file<P: AsRef<Path>, F: AsRef<Path>>(log_dir: P, file: F) -> Vec<Ping> {
    if let Ok(file) = File::open(log_dir.as_ref().join(file).as_os_str()) {
        let mut pings = BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok().and_then(|line| Ping::try_from(line).ok()))
            .collect::<Vec<_>>();
        pings.reverse();
        pings
    } else {
        println!("Error opening file\n");
        vec![]
    }
}

pub fn read_history<P: AsRef<Path>>(
    log_dir: P,
    offset: usize,
    count: usize,
    start: i64,
    end: i64,
) -> Vec<History> {
    let pings = read_log(log_dir, offset, count * 60, start, end);
    generate_history(&pings[..])
}

fn generate_history(log: &[Ping]) -> Vec<History> {
    let max_count = 3 * 24;

    let mut chunks: Vec<History> = vec![];
    if log.len() > 0 {
        let mut start = 0;
        let mut end = 0;
        let mut until = log[0].time / 3600 * 3600;

        for l in log {
            let mut i = 0;
            while l.time < until && i < max_count {
                chunks.push(History::from((&log[start..end], until)));

                start = end;
                until -= 3600;
                i += 1;
            }
            if i >= max_count {
                break;
            }
            end += 1;
        }
        if end > start {
            chunks.push(History::from((&log[start..end], until)));
        }
    }

    chunks
}

#[cfg(test)]
mod test {
    use super::*;

    use std::f64::NAN;

    #[test]
    fn test_generate_history() {
        let log = [Ping::new(1536062893, 10.0), Ping::new(1536059293, 20.0)];

        let history = generate_history(&log);

        assert_eq!(2, history.len());
        assert_eq!(History::new(1536062400, 10.0, 10.0, 10.0, 0.0, 1), history[0]);
        assert_eq!(History::new(1536058800, 20.0, 20.0, 20.0, 0.0, 1), history[1]);

        let log = [Ping::new(1536062893, 10.0), Ping::new(1536055693, 20.0)];

        let history = generate_history(&log);
        println!("{:?}", history);

        assert_eq!(3, history.len());
        assert_eq!(History::new(1536062400, 10.0, 10.0, 10.0, 0.0, 1), history[0]);
        assert_eq!(History::new(1536058800, NAN, NAN, NAN, NAN, 0), history[1]);
        assert_eq!(History::new(1536055200, 20.0, 20.0, 20.0, 0.0, 1), history[2]);
    }
}
