use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use actix_files::{Files, NamedFile};
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::Deserialize;

use super::hw;
use super::mc;
use super::ping::History;
use super::ping_stats;

static HTML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/ping.html"));
static SCRIPT: &str = include_str!(concat!(env!("OUT_DIR"), "/ping.js"));

pub(crate) static STYLE: &str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));

struct State {
    log_dir: PathBuf,
    drop_dir: Option<PathBuf>,
    mc_hosts: Arc<RwLock<Vec<mc::Status>>>,
}

#[derive(Deserialize, Copy, Clone)]
struct TimeQuery {
    offset: Option<usize>,
    count: Option<usize>,
    start: Option<i64>,
    end: Option<i64>,
}

#[actix_web::get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(HTML)
}

#[actix_web::get("/static/style.css")]
async fn style() -> HttpResponse {
    HttpResponse::Ok().content_type("text/css").body(STYLE)
}

#[actix_web::get("/static/ping.js")]
async fn script() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(SCRIPT)
}

#[actix_web::get("/robots.txt")]
async fn robots() -> HttpResponse {
    HttpResponse::Ok().body("User-agent: *\nDisallow: /\n")
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

#[actix_web::get("/api/mc")]
async fn api_mc(state: web::Data<State>) -> HttpResponse {
    let mc_state = state.mc_hosts.read().unwrap();
    HttpResponse::Ok().json(&*mc_state)
}

#[actix_web::get("/api/hw")]
async fn api_hw() -> HttpResponse {
    HttpResponse::Ok().json(hw::Status::request())
}

#[actix_web::get("/drop/{hash}/{file}")]
async fn drop(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    state: web::Data<State>,
) -> Result<HttpResponse, Error> {
    if let Some(drop_dir) = &state.drop_dir {
        let (hash, file) = path.as_ref();
        let filepath = drop_dir
            .join(sanitize_filename::sanitize(hash))
            .join(sanitize_filename::sanitize(file));
        println!("Download {:?}", filepath);
        if filepath.exists() {
            NamedFile::open(filepath)?.into_response(&req)
        } else {
            Err(io::Error::from(io::ErrorKind::NotFound).into())
        }
    } else {
        Err(io::Error::from(io::ErrorKind::NotFound).into())
    }
}

/// Starts the ping log webserver on the given `ip`
pub async fn run(
    ip: SocketAddr,
    log_dir: PathBuf,
    mc_hosts: Arc<RwLock<Vec<mc::Status>>>,
    drop_dir: Option<PathBuf>,
) -> std::io::Result<()> {
    println!("Ping server is running on {}", ip);

    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(middleware::Compress::default())
            .data(State {
                log_dir: log_dir.clone(),
                drop_dir: drop_dir.clone(),
                mc_hosts: mc_hosts.clone(),
            })
            .service(index)
            .service(style)
            .service(script)
            .service(robots)
            .service(api_pings)
            .service(api_history)
            .service(api_files)
            .service(api_mc)
            .service(api_hw)
            .service(Files::new("/api/files", log_dir.clone()));
        if drop_dir.is_some() {
            app = app.service(drop);
        }
        app
    })
    .bind(ip)
    .expect("Could not configure server")
    .run()
    .await
}
