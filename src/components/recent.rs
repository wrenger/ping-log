use chrono::offset::LocalResult;
use chrono::{Local, TimeZone};
use dioxus::prelude::*;

use crate::components::line_plot::{Line, LinePlot};
use crate::ping::Ping;

#[component]
pub fn RecentRecent(pings: Vec<Ping>) -> Element {
    pings.reverse();

    let lines = vec![Line {
        color: "oklch(var(--p))".into(),
        points: pings.iter().map(|p| (p.time as _, p.ping as _)).collect(),
    }];

    rsx! {
        div { class: "card bg-neutral shadow-xl container mx-auto",
            div { class: "card-body",
                h2 { class: "card-title", "Last Hour" }
                LinePlot {
                    lines: lines,
                    min_y: 0.0,
                    max_y: 50.0,
                    x_gran: 60.0,
                    x_trans: |x: f64| {
                        if let LocalResult::Single(dt) = Local.timestamp_opt(x as i64, 0) {
                            dt.format("%H:%M").to_string()
                        } else {
                            x.to_string()
                        }
                    },
                }
            }
        }
    }
}
