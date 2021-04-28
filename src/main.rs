//! # Ping Log
//! Simple RESTful webserver for logging and visualizing access times to a specified
//! host.
//! It is designed for a raspberry pi or other linux based IoT device running
//! permanently inside the network.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use chrono::{Local, Timelike};
use structopt::StructOpt;

mod drops;
mod hw;
mod mc;
mod ping;
mod ping_request;
mod ping_stats;
mod server;

/// Command line options
#[derive(Debug, StructOpt)]
#[structopt(
    name = "ping-log",
    about = "Simple RESTful webserver for logging and visualizing network access times."
)]
struct Opt {
    /// Time between ping requests
    #[structopt(short, long, default_value = "60")]
    interval: u32,

    /// Address or url of the ping target server
    #[structopt(short, long, default_value = "1.1.1.1")]
    ping_host: String,

    /// Filepath to the loggin directory
    #[structopt(short, long, parse(from_os_str), default_value = "log")]
    logs: PathBuf,

    /// Address and port of this webserver
    #[structopt(short, long, default_value = "127.0.0.1:8081")]
    web_host: SocketAddr,

    /// Address and port of this webserver
    #[structopt(short, long)]
    mc_hosts: Vec<String>,

    /// Filepath to the drop directory
    #[structopt(long, parse(from_os_str))]
    drop_dir: Option<PathBuf>,

    /// Address and port of the drop server
    #[structopt(long, default_value = "127.0.0.1:8082")]
    drop_host: SocketAddr,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    {
        // Ping reqest thread
        let log_dir = opt.logs.clone();
        let interval = opt.interval;
        let ping_host = opt.ping_host;

        thread::spawn(move || loop {
            let current_seconds = Local::now().second();
            thread::sleep(Duration::from_secs((interval - current_seconds) as u64));

            ping_request::ping_request(&ping_host, &log_dir);
        });
    };

    let mc_state = Arc::new(RwLock::new(Vec::new()));
    if !opt.mc_hosts.is_empty() {
        // Server status thread
        let interval = opt.interval;
        let mc_hosts = opt.mc_hosts.clone();
        let mc_state = mc_state.clone();

        thread::spawn(move || loop {
            mc::Status::refresh(&mc_state, &mc_hosts);

            let current_seconds = Local::now().second();
            thread::sleep(Duration::from_secs((interval - current_seconds) as u64));
        });
    };
    let main_server = server::run(opt.web_host, opt.logs, mc_state, opt.drop_dir.clone());

    if let Some(drop_dir) = opt.drop_dir {
        let drop_server = drops::run(opt.drop_host, drop_dir);
        // Run both at the same time
        let (first, second) = futures::join!(main_server, drop_server);
        first.and(second)
    } else {
        main_server.await
    }
}
