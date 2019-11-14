mod ping;
mod ping_request;
mod ping_server;
mod ping_stats;

use std::thread;
use std::time::Duration;
use std::net::SocketAddr;
use std::path::PathBuf;

use chrono::{Local, Timelike};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ping-log", about = "Simple RESTful webserver for logging and visualizing network access times.")]
struct Opt {
    #[structopt(short, long, default_value = "60")]
    interval: u32,

    #[structopt(short, long, default_value = "1.1.1.1")]
    ping_host: String,

    #[structopt(short, long, parse(from_os_str), default_value = "log")]
    logs: PathBuf,

    #[structopt(short, long, default_value = "127.0.0.1:8081")]
    web_host: SocketAddr,
}


fn main() {
    let opt = Opt::from_args();

    {
        let log_dir = opt.logs.clone();
        let interval = opt.interval;
        let ping_host = opt.ping_host;

        thread::spawn(move || loop {
            let current_seconds = Local::now().second();
            thread::sleep(Duration::from_secs((interval - current_seconds) as u64));

            ping_request::ping_request(&ping_host, &log_dir);
        });
    }
    ping_server::run_webserver(opt.web_host, opt.logs);
}
