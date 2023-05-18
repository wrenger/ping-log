use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use serde::Deserialize;
use warp::Filter;

use super::hw;
use super::mc;
use super::ping_stats;

struct State {
    log_dir: PathBuf,
    mc_hosts: Arc<RwLock<Vec<mc::Status>>>,
}

#[derive(Deserialize, Copy, Clone)]
#[serde(default)]
struct TimeQuery {
    offset: usize,
    count: usize,
    start: i64,
    end: i64,
}
impl Default for TimeQuery {
    fn default() -> Self {
        Self {
            offset: 0,
            count: 60,
            start: 0,
            end: 0,
        }
    }
}

/// Starts the ping log webserver on the given `ip`
pub async fn run(
    ip: SocketAddr,
    log_dir: PathBuf,
    web_dir: PathBuf,
    mc_hosts: Arc<RwLock<Vec<mc::Status>>>,
) {
    println!("Ping server is running on {ip}");

    let state = Arc::new(State { log_dir, mc_hosts });

    let with_state = warp::any().map(move || state.clone());

    let pings = warp::path("pings")
        .and(with_state.clone())
        .and(warp::query::<TimeQuery>())
        .map(|state: Arc<State>, query: TimeQuery| {
            warp::reply::json(&ping_stats::read_log(
                &state.log_dir,
                query.offset,
                query.count,
                query.start,
                query.end,
            ))
        });

    let hw = warp::path("hw").map(|| warp::reply::json(&hw::Status::request()));

    let mc = warp::path("mc")
        .and(with_state.clone())
        .map(|state: Arc<State>| {
            let mc_state = state.mc_hosts.read().unwrap();
            warp::reply::json(&*mc_state)
        });

    let api = warp::path("api").and(pings.or(hw).or(mc));

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(web_dir.join("index.html")));

    let routes = warp::get()
        .and(index.or(api).or(warp::fs::dir(web_dir)))
        .with(warp::compression::gzip());

    warp::serve(routes).run(ip).await
}
