use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Ping data (timestamp and duration in ms)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ping {
    pub time: i64,
    pub ping: f64,
}

impl Ping {
    pub fn new(time: i64, ping: f64) -> Ping {
        Ping { time, ping }
    }
}

impl From<(i64, f64)> for Ping {
    fn from(t: (i64, f64)) -> Self {
        Ping::new(t.0, t.1)
    }
}

impl fmt::Display for Ping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:.1}", self.time, self.ping)
    }
}

impl FromStr for Ping {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (t, p) = s.split_once(char::is_whitespace).ok_or(())?;
        Ok(Ping {
            time: t.parse().map_err(|_| ())?,
            ping: p.parse().map_err(|_| ())?,
        })
    }
}
