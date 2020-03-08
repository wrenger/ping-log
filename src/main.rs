//! # Ping Log
//! Simple RESTful webserver for logging and visualizing access times to a specified
//! host.
//! It is designed for a raspberry pi or other unix based IoT device running
//! permanently inside the network.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use chrono::{Local, Timelike};
use structopt::StructOpt;

mod ping;
mod ping_request;
mod ping_server;
mod ping_stats;

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
}

#[actix_rt::main]
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
    ping_server::run_webserver(opt.web_host, opt.logs).await
}
