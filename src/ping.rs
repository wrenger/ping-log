use std::convert::TryFrom;
use std::f64::NAN;

use serde::Serialize;

/// Ping data (timestamp and duration in ms)
#[derive(Debug, Clone, Serialize)]
pub struct Ping {
    pub time: i64,
    pub ping: f64,
}

impl Ping {
    pub fn new(time: i64, ping: f64) -> Ping {
        Ping { time, ping }
    }
}

impl TryFrom<String> for Ping {
    type Error = ();

    fn try_from(val: String) -> Result<Ping, ()> {
        if let Some(idx) = val.find(' ') {
            let (time, ping) = val.split_at(idx);
            if let Ok(time) = time.parse::<i64>() {
                if let Ok(ping) = ping[1..].parse::<f64>() {
                    return Ok(Ping::new(time, ping));
                }
            }
        }
        Err(())
    }
}

/// Accumulated `Ping`s over a certain period of time
#[derive(Debug, Clone, Serialize)]
pub struct History {
    pub time: i64,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub lost: f64,
    pub count: u32,
}

impl History {
    pub fn new(time: i64, min: f64, max: f64, avg: f64, lost: f64, count: u32) -> History {
        History {
            time,
            min,
            max,
            avg,
            lost,
            count,
        }
    }

    pub fn from(time: i64, pings: &[Ping]) -> History {
        let mut min = 1000.0;
        let mut max = 0.0;
        let mut sum = 0.0;
        let mut lost = 0.0;

        for entry in pings {
            if entry.ping < min {
                min = entry.ping;
            }
            if entry.ping >= 1000.0 {
                lost += 1.0;
            } else {
                if entry.ping > max {
                    max = entry.ping;
                }
                sum += entry.ping;
            }
        }

        let avg = (1000.0 * sum / (pings.len() as f64 - lost)).round() / 1000.0;
        let lost = (1000.0 * lost / pings.len() as f64).round() / 1000.0;

        if min >= 1000.0 {
            min = NAN;
        }
        if max <= 0.0 {
            max = NAN;
        }

        History::new(time, min, max, avg, lost, pings.len() as u32)
    }
}

impl PartialEq for History {
    fn eq(&self, other: &History) -> bool {
        self.time == other.time
            && (self.min == other.min || (self.min.is_nan() && other.min.is_nan()))
            && (self.max == other.max || (self.max.is_nan() && other.max.is_nan()))
            && (self.avg == other.avg || (self.avg.is_nan() && other.avg.is_nan()))
            && (self.lost == other.lost || (self.lost.is_nan() && other.lost.is_nan()))
            && self.count == other.count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_history_from() {
        // No pings
        let log = vec![];
        let generated = History::from(0_i64, &log[..]);
        assert_eq!(History::new(0, NAN, NAN, NAN, NAN, 0), generated);

        // Single ping
        let log = vec![Ping::new(0, 20.0)];
        let generated = History::from(0_i64, &log[..]);
        assert_eq!(History::new(0, 20.0, 20.0, 20.0, 0.0, 1), generated);

        // 100% lost
        let log = vec![Ping::new(0, 1000.0)];
        let generated = History::from(0_i64, &log[..]);
        assert_eq!(History::new(0, NAN, NAN, NAN, 1.0, 1), generated);

        // Multiple pings
        let log = vec![
            Ping::new(0, 40.0),
            Ping::new(0, 20.0),
            Ping::new(0, 30.0),
            Ping::new(0, 1000.0),
        ];
        let generated = History::from(0_i64, &log[..]);
        assert_eq!(History::new(0, 20.0, 40.0, 30.0, 0.25, 4), generated);
    }
}
