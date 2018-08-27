extern crate chrono;
extern crate argparse;

mod ping_web;
mod ping_stats;
mod ping_request;

use std::thread;
use std::time::{Duration};

use self::chrono::prelude::*;
use argparse::{ArgumentParser, Store};


fn main() {
    let mut interval = 60;
    let mut ping_host = String::from("8.8.8.8");
    let mut web_host = String::from("127.0.0.1:8081");
    let mut log_dir = String::from("./log");
    let mut web_dir = String::from("./www");

    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Simple RESTful webserver for logging and visualizing network access times.");
        parser.refer(&mut interval).add_option(&["-i", "--interval"], Store, "Ping Interval");
        parser.refer(&mut ping_host).add_option(&["-p", "--ping-host"], Store, "Host for ping requests");
        parser.refer(&mut log_dir).add_option(&["-l", "--log"], Store, "Directory for the log files");
        parser.refer(&mut web_host).add_option(&["-h", "--web-host"], Store, "Host ip for the webserver");
        parser.refer(&mut web_dir).add_option(&["-w", "--web"], Store, "Web server root directory");
    }

    {
        let ping_host = ping_host.clone();
        let log_dir = log_dir.clone();
        let web_dir = web_dir.clone();

        thread::spawn(move || {
            loop {
                ping_request::ping_request(&ping_host, &log_dir);
                ping_stats::generate_statistics(&log_dir, &web_dir);

                let current_seconds = Local::now().second();
                thread::sleep(Duration::from_secs((interval - current_seconds) as u64));
            }
        });
    }
    let web_host = web_host.parse().unwrap();
    ping_web::run_webserver(web_host, &web_dir, &log_dir);
}

