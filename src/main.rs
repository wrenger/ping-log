#[cfg(feature = "server")]
use std::net::SocketAddr;
#[cfg(feature = "server")]
use std::path::PathBuf;
#[cfg(feature = "server")]
use std::sync::OnceLock;
use std::time::Duration;

#[cfg(feature = "server")]
use clap::Parser;

use components::history::PingHistory;
use components::hw::{Hardware, HardwareProps};
use components::recent::RecentRecent;
use components::stats::{PingStats, Stats};
use dioxus::prelude::*;
use ping::Ping;
use server_fn::codec::GetUrl;
use util::sleep;

mod components;
mod ping;
#[cfg(feature = "server")]
mod server;
mod util;

#[cfg(feature = "server")]
static LOG_DIR: OnceLock<PathBuf> = OnceLock::new();

#[cfg(feature = "server")]
/// Command line options
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Time between ping requests
    #[arg(short, long, default_value_t = 60)]
    interval: u64,

    /// Address or url of the ping target server
    #[arg(short, long, default_value = "1.1.1.1")]
    ping_host: String,

    /// Filepath to the loggin directory
    #[arg(short, long, default_value = "log")]
    logs: PathBuf,

    /// Filepath to the web directory
    ///
    /// Default: <executable path>/public
    #[arg(long)]
    web: Option<PathBuf>,

    /// Address and port of this webserver
    #[arg(short, long, default_value = "127.0.0.1:8081")]
    web_host: SocketAddr,
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::logger::initialize_default();
    LaunchBuilder::new().launch(App);
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    dioxus::logger::initialize_default();
    use dioxus::fullstack::UnableToLoadIndex;
    use tower_http::compression::CompressionLayer;
    use tracing::warn;

    let args = Args::parse();

    LOG_DIR.set(args.logs.clone()).unwrap();

    {
        // Ping reqest thread
        let log_dir = args.logs.clone();
        let interval = args.interval;
        let ping_host = args.ping_host.clone();
        tokio::spawn(
            async move { server::ping_request::monitor(&ping_host, interval, &log_dir).await },
        );
    };

    let web = args.web.unwrap_or_else(|| {
        std::env::current_exe()
            .expect("Failed to get current executable path")
            .parent()
            .unwrap()
            .join("public")
    });
    let config = ServeConfig::builder()
        .index_path(web.join("index.html"))
        .build();

    // Get the address the server should run on. If the CLI is running, the CLI proxies fullstack into the main address
    // and we use the generated address the CLI gives us
    let address = if let (Some(host), Some(port)) = (
        dioxus_cli_config::server_ip(),
        dioxus_cli_config::server_port(),
    ) {
        SocketAddr::new(host, port)
    } else {
        args.web_host
    };

    struct TryIntoResult(Result<ServeConfig, UnableToLoadIndex>);
    impl TryInto<ServeConfig> for TryIntoResult {
        type Error = UnableToLoadIndex;
        fn try_into(self) -> Result<ServeConfig, Self::Error> {
            self.0
        }
    }

    let router = axum::Router::new()
        .serve_dioxus_application(TryIntoResult(config), App)
        .layer(CompressionLayer::new());
    let router = router.into_make_service();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    warn!(
        "Running on: {address}\n(static={} log={} host={} i={})",
        web.display(),
        args.logs.display(),
        args.ping_host,
        args.interval
    );

    axum::serve(listener, router).await.unwrap();
}

const FAVICON: Asset = asset!("/assets/favicon.png");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    let mut reload = use_signal(|| 0);
    let recent = use_server_future(move || {
        reload(); // Respond to reload
        async move {
            let now = chrono::Local::now().timestamp();
            get_recent(0, 60, now, now - 60 * 60)
                .await
                .unwrap_or_default()
        }
    })?;
    let hw = use_server_future(move || {
        reload(); // Respond to reload
        async move { get_hw_status().await.unwrap_or_default() }
    })?;

    use_future(move || async move {
        loop {
            sleep(Duration::from_secs(30)).await;
            reload.set(0); // Trigger reload
        }
    });

    let (stats, pings) = recent().unwrap_or_default();
    let hw = hw().unwrap_or_default();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "space-y-5",
            div { class: "navbar bg-neutral",
                a { class: "btn btn-ghost text-xl",
                    href: "/",
                    "Ping Log"
                }
            }

            Stats { stats }

            RecentRecent { pings }

            PingHistory { }

            Hardware { ..hw }
        }
    }
}

#[server(endpoint = "/recent", input = GetUrl)]
async fn get_recent(
    offset: usize,
    count: usize,
    start: i64,
    end: i64,
) -> Result<(PingStats, Vec<Ping>), ServerFnError> {
    use server::ping_stats::*;
    let dir = LOG_DIR.get().unwrap();
    let logs = read_log(dir, offset, count, start, end);
    let stats = accumulate(&logs, 0);
    Ok((stats, logs))
}

#[server(endpoint = "/hw", input = GetUrl)]
async fn get_hw_status() -> Result<HardwareProps, ServerFnError> {
    Ok(server::hw::request())
}
