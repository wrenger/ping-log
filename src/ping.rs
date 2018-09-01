extern crate chrono;

use self::chrono::prelude::*;

#[derive(Debug)]
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

    pub fn time_string(&self) -> String {
        let loc = self.date_time();
        loc.format("%d.%m.%y %H:%M").to_string()
    }

    pub fn tuple(&self) -> (String, f64) {
        (self.time_string(), self.latency)
    }
}

impl Clone for Ping {
    fn clone(&self) -> Ping {
        Ping::new(self.time, self.latency)
    }
}
