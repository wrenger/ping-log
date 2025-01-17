//! # Ping Log
//! Simple RESTful webserver for logging and visualizing access times to a specified
//! host.
//! It is designed for a raspberry pi or other linux based IoT device running
//! permanently inside the network.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use clap::Parser;

mod hw;
mod mc;
mod ping;
mod ping_request;
mod ping_stats;
mod server;

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
    #[arg(long, default_value = "ping-view/build")]
    web: PathBuf,

    /// Address and port of this webserver
    #[arg(short, long, default_value = "127.0.0.1:8081")]
    web_host: SocketAddr,

    /// Address and port of this webserver
    #[arg(short, long)]
    mc_hosts: Vec<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    {
        // Ping reqest thread
        let log_dir = args.logs.clone();
        let interval = args.interval;
        let ping_host = args.ping_host;

        tokio::spawn(async move { ping_request::monitor(&ping_host, interval, &log_dir).await });
    };

    let mc_state = Arc::new(RwLock::new(Vec::new()));
    if !args.mc_hosts.is_empty() {
        // Server status thread
        let interval = args.interval;
        let mc_hosts = args.mc_hosts.clone();
        let mc_state = mc_state.clone();

        tokio::spawn(async move {
            loop {
                mc::Status::refresh(&mc_state, &mc_hosts).await;

                let epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                let next =
                    Duration::from_secs(((epoch.as_secs() + interval) / interval) * interval);

                tokio::time::sleep(next - epoch).await;
            }
        });
    };
    server::run(args.web_host, args.logs, args.web, mc_state).await
}
