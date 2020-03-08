use std::net::SocketAddr;
use std::path::PathBuf;

use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::Deserialize;

use super::ping::History;
use super::ping_stats;

struct State {
    log_dir: PathBuf,
}

#[derive(Deserialize, Copy, Clone)]
struct TimeQuery {
    offset: Option<usize>,
    count: Option<usize>,
    start: Option<i64>,
    end: Option<i64>,
}

#[actix_web::get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

#[actix_web::get("/api/pings")]
async fn api_pings(query: web::Query<TimeQuery>, state: web::Data<State>) -> HttpResponse {
    let pings = ping_stats::read_log(
        &state.log_dir,
        query.offset.unwrap_or(0),
        query.count.unwrap_or(60),
        query.start.unwrap_or(0),
        query.end.unwrap_or(0),
    );
    HttpResponse::Ok().json((
        History::from(pings.first().map_or(0, |p| p.time), &pings),
        pings,
    ))
}

#[actix_web::get("/api/history")]
async fn api_history(query: web::Query<TimeQuery>, state: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().json(ping_stats::read_history(
        &state.log_dir,
        query.offset.unwrap_or(0),
        query.count.unwrap_or(24),
        query.start.unwrap_or(0),
        query.end.unwrap_or(0),
    ))
}

#[actix_web::get("/api/files")]
async fn api_files(state: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().json(ping_stats::log_files(&state.log_dir))
}

/// Starts the ping log webserver on the given `ip`
pub async fn run_webserver(ip: SocketAddr, log_dir: PathBuf) -> std::io::Result<()> {
    println!("Server is running on {}", ip);

    HttpServer::new(move || {
        App::new()
            .data(State {
                log_dir: log_dir.clone(),
            })
            .service(index)
            .service(api_pings)
            .service(api_history)
            .service(api_files)
            .service(Files::new("/api/file", log_dir.clone()))
            .service(Files::new("/static", "./static"))
    })
    .bind(ip)
    .expect("Could not configure server")
    .run()
    .await
}
