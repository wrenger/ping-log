use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use rocket::config::{Config, Environment};
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::json::Json;

use super::ping::{History, Ping};
use super::ping_stats;

#[get("/")]
fn index() -> Option<NamedFile> {
    static_resource(PathBuf::from("index.html"))
}

#[get("/static/<path..>")]
fn static_resource(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

#[get("/api/pings?<offset>&<count>&<start>&<end>")]
fn api_pings(
    offset: Option<usize>,
    count: Option<usize>,
    start: Option<i64>,
    end: Option<i64>,
    log_dir: State<String>,
) -> Json<Vec<Ping>> {
    Json(ping_stats::read_log(
        log_dir.as_str(),
        offset.unwrap_or(0),
        count.unwrap_or(60),
        start.unwrap_or(0),
        end.unwrap_or(0),
    ))
}

#[get("/api/history?<offset>&<count>&<start>&<end>")]
fn api_history(
    offset: Option<usize>,
    count: Option<usize>,
    start: Option<i64>,
    end: Option<i64>,
    log_dir: State<String>,
) -> Json<Vec<History>> {
    Json(ping_stats::read_history(
        log_dir.as_str(),
        offset.unwrap_or(0),
        count.unwrap_or(24),
        start.unwrap_or(0),
        end.unwrap_or(0),
    ))
}


#[get("/api/files")]
fn api_files(log_dir: State<String>) -> Json<Vec<String>> {
    Json(ping_stats::log_files(log_dir.as_str()))
}

#[get("/api/file/<file..>")]
fn api_file(file: PathBuf, log_dir: State<String>) -> Option<NamedFile> {
    NamedFile::open(Path::new(log_dir.as_str()).join(file)).ok()
}


pub fn run_webserver(ip: SocketAddr, log_dir: &String) {
    println!("Server is running on {}", ip);

    let config = Config::build(Environment::Development)
        .address(ip.ip().to_string())
        .port(ip.port())
        .expect("Rocket Config Error");

    let log_dir = log_dir.clone();

    rocket::custom(config)
        .mount(
            "/",
            routes![
                index,
                static_resource,
                api_pings,
                api_history,
                api_files,
                api_file,
            ],
        )
        .manage(log_dir)
        .launch();
}

