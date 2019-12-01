use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpResponse, HttpServer};
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

// "/"
fn index() -> Option<NamedFile> {
    static_resource(PathBuf::from("index.html"))
}

// "/static/<path..>"
fn static_resource(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

// "/api/pings?<offset>&<count>&<start>&<end>"
fn api_pings(query: web::Query<TimeQuery>, state: web::Data<State>) -> HttpResponse {
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

// "/api/history?<offset>&<count>&<start>&<end>"
fn api_history(query: web::Query<TimeQuery>, state: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().json(ping_stats::read_history(
        &state.log_dir,
        query.offset.unwrap_or(0),
        query.count.unwrap_or(24),
        query.start.unwrap_or(0),
        query.end.unwrap_or(0),
    ))
}

// "/api/files"
fn api_files(state: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().json(ping_stats::log_files(&state.log_dir))
}

/// Starts the ping log webserver on the given `ip`
pub fn run_webserver(ip: SocketAddr, log_dir: PathBuf) {
    println!("Server is running on {}", ip);

    HttpServer::new(move || {
        App::new()
            .data(State {
                log_dir: log_dir.clone(),
            })
            .route("/", web::get().to(index))
            .route("/api/pings", web::get().to(api_pings))
            .route("/api/history", web::get().to(api_history))
            .route("/api/files", web::get().to(api_files))
            .service(Files::new("/api/file", log_dir.clone()))
            .service(Files::new("/static", "./static"))
    })
    .bind(ip)
    .expect("Could not configure server")
    .run()
    .unwrap()
}
