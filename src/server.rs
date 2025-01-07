use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use axum::body::Body;
use axum::error_handling::HandleErrorLayer;
use axum::extract::{Json, Query, State};
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::BoxError;
use serde::Deserialize;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::error;

use super::hw;
use super::mc;
use super::ping_stats;

struct AppState {
    log_dir: PathBuf,
    mc_hosts: Arc<RwLock<Vec<mc::Status>>>,
    web_dir: PathBuf,
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

    let app = axum::Router::new()
        .route("/api/pings", get(handle_pings))
        .route("/api/hw", get(handle_hw))
        .route("/api/mc", get(handle_mc))
        .route("/", get(serve_index))
        .fallback_service(ServeDir::new(&web_dir))
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new())
                .layer(HandleErrorLayer::new(error_handler))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(Arc::new(AppState {
            log_dir,
            mc_hosts,
            web_dir,
        }));

    axum::serve(tokio::net::TcpListener::bind(ip).await.unwrap(), app)
        .await
        .unwrap();
}

async fn error_handler(error: BoxError) -> StatusCode {
    if error.is::<tower::timeout::error::Elapsed>() {
        StatusCode::REQUEST_TIMEOUT
    } else {
        error!("Internal server error: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn handle_pings(
    State(state): State<Arc<AppState>>,
    Query(query): Query<TimeQuery>,
) -> Json<Vec<super::ping::Ping>> {
    Json(ping_stats::read_log(
        &state.log_dir,
        query.offset,
        query.count,
        query.start,
        query.end,
    ))
}

async fn handle_hw() -> Json<hw::Status> {
    Json(hw::Status::request())
}

async fn handle_mc(State(state): State<Arc<AppState>>) -> Json<Vec<mc::Status>> {
    let mc_state = state.mc_hosts.read().unwrap();
    Json(mc_state.clone())
}

async fn serve_index(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
) -> impl axum::response::IntoResponse {
    ServeFile::new(state.web_dir.join("index.html"))
        .oneshot(req)
        .await
        .unwrap()
        .into_response()
}
