extern crate futures;
extern crate hyper;

use self::hyper::rt::Future;
use self::hyper::{service, Body, Request, Response, Server, StatusCode};

use self::futures::future;

use std::fs::read_to_string;
use std::net::SocketAddr;
use std::path::Path;

mod content_type {
    pub const HTML: &str = "text/html; charset=utf-8";
    pub const JS: &str = "application/javascript; charset=utf-8";
    pub const JSON: &str = "application/json; charset=utf-8";
    pub const CSS: &str = "text/css; charset=utf-8";
    pub const TXT: &str = "text/plain; charset=utf-8";
}

pub fn run_webserver(ip: SocketAddr, web_dir: &String, log_dir: &String) {
    let web_dir = web_dir.clone();
    let log_dir = log_dir.clone();

    println!("Server is running on {}", ip);
    hyper::rt::run(future::lazy(move || {
        let service = move || {
            let web_dir = web_dir.clone();
            let log_dir = log_dir.clone();

            service::service_fn_ok(move |r| router(r, &web_dir, &log_dir))
        };

        let server = Server::bind(&ip)
            .serve(service)
            .map_err(|e| eprintln!("server error: {}", e));

        server
    }));
}

fn router(request: Request<Body>, web_dir: &String, log_dir: &String) -> Response<Body> {
    let web_dir = web_dir.clone();

    println!("Request: {}: {}", request.method(), request.uri().path());

    match request.uri().path() {
        "/" => Response::builder()
            .header("Content-type", content_type::HTML)
            .body(resource(web_dir + "/index.html"))
            .unwrap(),
        "/ping-web.js" => Response::builder()
            .header("Content-type", content_type::JS)
            .body(resource(web_dir + "/ping-web.js"))
            .unwrap(),
        "/style.css" => Response::builder()
            .header("Content-type", content_type::CSS)
            .body(resource(web_dir + "/style.css"))
            .unwrap(),
        "/data.json" => Response::builder()
            .header("Content-type", content_type::JSON)
            .body(resource(web_dir + "/data.json"))
            .unwrap(),
        path => try_open_log(path, log_dir.as_str()),
    }
}

fn resource<P: AsRef<Path>>(path: P) -> Body {
    read_to_string(path)
        .map(|s| Body::from(s))
        .unwrap_or(Body::empty())
}

fn try_open_log(path: &str, log_dir: &str) -> Response<Body> {
    let log_file = path.split_at(1).1;
    let files = super::ping_stats::log_files(log_dir);

    if files.contains(&log_file.to_string()) {
        Response::builder()
            .header("Content-type", content_type::TXT)
            .body(resource(log_file))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()
    }
}
