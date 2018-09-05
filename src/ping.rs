use chrono::{DateTime, Local, NaiveDateTime, Utc};

use std::f64::NAN;

#[derive(Debug, Clone, Serialize)]
pub struct Ping {
    pub time: i64,
    pub latency: f64,
}

impl Ping {
    pub fn new(time: i64, latency: f64) -> Ping {
        Ping { time, latency }
    }

    pub fn date_time(&self) -> DateTime<Local> {
        let native = NaiveDateTime::from_timestamp(self.time, 0);
        let time: DateTime<Utc> = DateTime::from_utc(native, Utc);

        time.with_timezone(&Local)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct History {
    pub time: i64,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub lost: u32,
    pub count: u32,
}

impl History {
    pub fn new(time: i64, min: f64, max: f64, avg: f64, lost: u32, count: u32) -> History {
        History {
            time: time,
            min: min,
            max: max,
            avg: avg,
            lost: lost,
            count: count,
        }
    }
}

impl<'a> From<(&'a [Ping], i64)> for History {
    fn from(log: (&'a [Ping], i64)) -> History {
        let mut min = 1000.0;
        let mut max = 0.0;
        let mut sum = 0.0;
        let mut lost: u32 = 0;

        for entry in log.0 {
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

        let avg = ((sum * 100.0) / (log.0.len() - lost as usize) as f64).round() as f64 / 100.0;

        if min >= 1000.0 {
            min = NAN;
        }
        if max <= 0.0 {
            max = NAN;
        }

        History::new(log.1, min, max, avg, lost, log.0.len() as u32)
    }
}

impl PartialEq for History {
    fn eq(&self, other: &History) -> bool {
        self.time == other.time
            && (self.min == other.min || (self.min.is_nan() && other.min.is_nan()))
            && (self.max == other.max || (self.max.is_nan() && other.max.is_nan()))
            && (self.avg == other.avg || (self.avg.is_nan() && other.avg.is_nan()))
            && self.lost == other.lost
            && self.count == other.count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_history_from() {
        let log = vec![];
        let generated = History::from((&log[..], 0_i64));
        assert_eq!(History::new(0, NAN, NAN, NAN, 0, 0), generated);

        let log = vec![Ping::new(0, 20.0)];
        let generated = History::from((&log[..], 0_i64));
        assert_eq!(History::new(0, 20.0, 20.0, 20.0, 0, 1), generated);

        let log = vec![
            Ping::new(0, 40.0),
            Ping::new(0, 20.0),
            Ping::new(0, 30.0),
            Ping::new(0, 1000.0),
        ];
        let generated = History::from((&log[..], 0_i64));
        assert_eq!(History::new(0, 20.0, 40.0, 30.0, 1, 4), generated);
    }
}
