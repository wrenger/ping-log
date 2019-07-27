#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

extern crate argparse;
extern crate chrono;

mod ping;
mod ping_request;
mod ping_server;
mod ping_stats;

use std::thread;
use std::time::Duration;

use self::chrono::{Local, Timelike};
use argparse::{ArgumentParser, Store};


fn main() {
    let mut interval = 60;
    let mut ping_host = String::from("8.8.8.8");
    let mut web_host = String::from("127.0.0.1:8081");
    let mut log_dir = String::from("log");

    {
        let mut parser = ArgumentParser::new();
        parser.set_description(
            "Simple RESTful webserver for logging and visualizing network access times.",
        );
        parser.refer(&mut interval).add_option(
            &["-i", "--interval"],
            Store,
            "Ping interval in seconds",
        );
        parser.refer(&mut ping_host).add_option(
            &["-p", "--ping-host"],
            Store,
            "Host for ping requests",
        );
        parser.refer(&mut log_dir).add_option(
            &["-l", "--logs"],
            Store,
            "Directory for the log files",
        );
        parser.refer(&mut web_host).add_option(
            &["-w", "--web-host"],
            Store,
            "Host ip for the webserver",
        );
        parser.parse_args_or_exit();
    }
    let web_host = web_host.parse().expect("Invalid server host address!");

    {
        let ping_host = ping_host.clone();
        let log_dir = log_dir.clone();

        thread::spawn(move || loop {
            let current_seconds = Local::now().second();
            thread::sleep(Duration::from_secs((interval - current_seconds) as u64));

            ping_request::ping_request(&ping_host, &log_dir);
        });
    }
    ping_server::run_webserver(web_host, &log_dir);
}

