use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use serde::Deserialize;

use super::hw;
use super::mc;
use super::ping_stats;

struct State {
    log_dir: PathBuf,
    mc_hosts: Arc<RwLock<Vec<mc::Status>>>,
}

#[derive(Deserialize, Copy, Clone)]
struct TimeQuery {
    offset: Option<usize>,
    count: Option<usize>,
    start: Option<i64>,
    end: Option<i64>,
}

#[actix_web::get("/api/pings")]
async fn api_pings(query: web::Query<TimeQuery>, state: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().json(ping_stats::read_log(
        &state.log_dir,
        query.offset.unwrap_or(0),
        query.count.unwrap_or(60),
        query.start.unwrap_or(0),
        query.end.unwrap_or(0),
    ))
}

#[actix_web::get("/api/mc")]
async fn api_mc(state: web::Data<State>) -> HttpResponse {
    let mc_state = state.mc_hosts.read().unwrap();
    HttpResponse::Ok().json(&*mc_state)
}

#[actix_web::get("/api/hw")]
async fn api_hw() -> HttpResponse {
    HttpResponse::Ok().json(hw::Status::request())
}

/// Starts the ping log webserver on the given `ip`
pub async fn run(
    ip: SocketAddr,
    log_dir: PathBuf,
    web_dir: PathBuf,
    mc_hosts: Arc<RwLock<Vec<mc::Status>>>,
) -> std::io::Result<()> {
    println!("Ping server is running on {}", ip);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .data(State {
                log_dir: log_dir.clone(),
                mc_hosts: mc_hosts.clone(),
            })
            .service(api_pings)
            .service(api_mc)
            .service(api_hw)
            .service(actix_files::Files::new("/", &web_dir).index_file("index.html"))
    })
    .bind(ip)
    .expect("Could not configure server")
    .run()
    .await
}
