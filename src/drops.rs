use std::fs;
use std::io::{self, Write};
use std::net::SocketAddr;
use std::path::{Component, Path, PathBuf};

use futures::{StreamExt, TryStreamExt};

use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};

use serde::Serialize;

use crate::server::STYLE;

struct State {
    drop_dir: PathBuf,
}

#[derive(Serialize)]
struct FileDrop {
    url: String,
}

fn drop_url_from(file: &Path) -> Option<String> {
    let mut components = file.components();
    if let Some(Component::Normal(filename)) = components.next_back() {
        if let Some(filename) = filename.to_str() {
            if let Some(Component::Normal(hash)) = components.next_back() {
                if let Some(hash) = hash.to_str() {
                    return Some(format!("/drop/{}/{}", hash, filename));
                }
            }
        }
    }
    None
}

fn all_drops(dir: &Path) -> io::Result<Vec<String>> {
    let mut drops = vec![];
    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(Ok(entry)) = path.read_dir()?.next() {
                if let Some(drop) = drop_url_from(&entry.path()) {
                    drops.push(drop);
                }
            }
        }
    }
    Ok(drops)
}

static HTML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/drop.html"));
static SCRIPT: &str = include_str!(concat!(env!("OUT_DIR"), "/drop.js"));

#[actix_web::get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(HTML)
}

#[actix_web::get("/static/style.css")]
async fn style() -> HttpResponse {
    HttpResponse::Ok().content_type("text/css").body(STYLE)
}

#[actix_web::get("/static/drop.js")]
async fn script() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(SCRIPT)
}

#[actix_web::get("/robots.txt")]
async fn robots() -> HttpResponse {
    HttpResponse::Ok().body("User-agent: *\nDisallow: /\n")
}

#[actix_web::post("/upload")]
async fn upload(mut payload: Multipart, state: web::Data<State>) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    if let Ok(Some(mut field)) = payload.try_next().await {
        let tmp_path = PathBuf::from("tmp");
        if !tmp_path.exists() {
            let tmp_path = tmp_path.clone();
            // blocking operation, use threadpool
            web::block(|| fs::create_dir_all(tmp_path)).await?;
        }

        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let sanitized_filename = sanitize_filename::sanitize(filename);
        let tmp_file = tmp_path.join(&sanitized_filename);

        let tmp_file_mv = tmp_file.clone();
        // blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(tmp_file_mv))
            .await
            .unwrap();

        let mut hash = sha1::Sha1::new();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            hash.update(&data);
            // blocking operation, use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
        // Create directory to
        let new_path = PathBuf::from(&state.drop_dir).join(hash.digest().to_string());
        if !new_path.exists() {
            let new_file = new_path.join(sanitized_filename);
            // blocking operation, use threadpool
            {
                let new_file = new_file.clone();
                web::block(|| {
                    fs::create_dir_all(new_path).and_then(|_| fs::rename(tmp_file, new_file))
                })
                .await?;
            }
            Ok(HttpResponse::Ok().json(drop_url_from(&new_file)))
        } else {
            web::block(|| fs::remove_file(tmp_file)).await?;
            Err(io::Error::from(io::ErrorKind::AlreadyExists).into())
        }
    } else {
        Err(().into())
    }
}

#[actix_web::get("/list")]
async fn list(state: web::Data<State>) -> Result<HttpResponse, Error> {
    let drops = web::block(move || all_drops(&state.drop_dir)).await?;
    Ok(HttpResponse::Ok().json(drops))
}

#[actix_web::delete("/remove/drop/{hash}/{filename}")]
async fn remove(
    path: web::Path<(String, String)>,
    state: web::Data<State>,
) -> Result<HttpResponse, Error> {
    let filename: PathBuf = [
        sanitize_filename::sanitize(&path.0),
        sanitize_filename::sanitize(&path.1),
    ]
    .iter()
    .collect();
    println!("Remove {:?}", filename);
    let path = state.drop_dir.join(filename);
    if path.exists() {
        web::block(move || fs::remove_dir_all(path.parent().unwrap())).await?;
        Ok(HttpResponse::Ok().into())
    } else {
        Err(io::Error::from(io::ErrorKind::NotFound).into())
    }
}

/// Starts the drop webserver on the given `ip`
pub async fn run(ip: SocketAddr, drop_dir: PathBuf) -> std::io::Result<()> {
    println!("Drop Server is running on {}", ip);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .data(State {
                drop_dir: drop_dir.clone(),
            })
            .service(index)
            .service(style)
            .service(script)
            .service(robots)
            .service(upload)
            .service(list)
            .service(remove)
    })
    .bind(ip)
    .expect("Could not configure server")
    .run()
    .await
}
