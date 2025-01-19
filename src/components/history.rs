use std::time::Duration;

use chrono::offset::LocalResult;
use chrono::{Local, NaiveDate, NaiveDateTime, TimeZone};
use dioxus::prelude::*;
use server_fn::codec::GetUrl;

use crate::components::line_plot::{Line, LinePlot};
use crate::util::sleep;

use super::stats::PingStats;

pub const LOG_DURATION: chrono::Duration = chrono::Duration::weeks(8);

#[component]
pub fn PingHistory() -> Element {
    let mut date = use_signal(|| Local::now().format("%Y-%m-%d").to_string());
    let history = use_server_future(move || {
        let end = NaiveDate::parse_from_str(&date(), "%Y-%m-%d").unwrap();
        let end = NaiveDateTime::from(end);
        let end = Local.from_local_datetime(&end).unwrap().timestamp();
        let start = end + 24 * 60 * 60;
        async move { get_stats(0, 60 * 24, start, end).await.unwrap_or_default() }
    })?;

    use_future(move || async move {
        loop {
            sleep(Duration::from_secs(30)).await;
            date.set(date());
        }
    });

    let history = history().unwrap_or_default();
    let lines = vec![
        Line {
            color: "oklch(var(--su))".into(),
            points: history.iter().map(|s| (s.time as _, s.min)).collect(),
        },
        Line {
            color: "oklch(var(--wa))".into(),
            points: history.iter().map(|s| (s.time as _, s.max)).collect(),
        },
        Line {
            color: "oklch(var(--p))".into(),
            points: history.iter().map(|s| (s.time as _, s.avg)).collect(),
        },
        Line {
            color: "oklch(var(--er))".into(),
            points: history
                .iter()
                .map(|s| (s.time as _, 50.0 * s.lost as f64 / s.count.max(1) as f64))
                .collect(),
        },
    ];

    rsx! {
        div { class: "card bg-neutral shadow-xl container mx-auto",
            div { class: "card-body",
                div {
                    class: "card-title flex-row justify-between",
                    h2 { "History" }
                    input { type: "date",
                        class: "input input-bordered input-sm",
                        value: date,
                        max: Local::now().format("%Y-%m-%d").to_string(),
                        min: (Local::now() - LOG_DURATION).format("%Y-%m-%d").to_string(),
                        onchange: move |e| date.set(e.value())
                    }
                }
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

#[server(endpoint = "/history", input = GetUrl)]
async fn get_stats(
    offset: usize,
    count: usize,
    start: i64,
    end: i64,
) -> Result<Vec<PingStats>, ServerFnError> {
    use crate::server::ping_stats::*;
    let dir = crate::LOG_DIR.get().unwrap();
    // logs in reverse order
    let mut logs = read_log(dir, offset, count, start, end);
    logs.reverse();
    // accumulate every hour
    let hour = 60 * 60;
    let mut stats: Vec<PingStats> = vec![];
    for chunk in logs.chunk_by(|a, b| a.time / hour == b.time / hour) {
        // fill in missing hours
        if let Some(last) = stats.last() {
            let last_time = last.time / hour;
            for h in last_time + 1..chunk[0].time / hour {
                stats.push(PingStats {
                    time: h * hour,
                    ..Default::default()
                });
            }
        }
        stats.push(accumulate(chunk, chunk[0].time / hour * hour));
    }

    stats.reverse();
    Ok(stats)
}
