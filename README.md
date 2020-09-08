# Ping Log

Simple RESTful webserver for logging and visualizing access times to a specified
host.
It is designed for a raspberry pi or other unix based IoT device running
permanently inside the network.

It is build using [Rust](https://www.rust-lang.org/) and the
[actix](https://actix.rs/) framework.

## Setup

First [Rust](https://www.rust-lang.org/learn/get-started) has to be installed.

**Build & Execute:**

```bash
cargo run
```

> Cross-compiling can be done with [cross](https://github.com/rust-embedded/cross)
>
> E.g. for the raspberry pi:
> 32bit: `cross build --target armv7-unknown-linux-gnueabihf`
> 64bit: `cross build --target aarch64-unknown-linux-gnu`

**Optional arguments:**
```bash
cargo run -- <args>
```

| Argument                 | Description                        |
|--------------------------|------------------------------------|
| -h,--help                | Show this help message and exit    |
| -i,--interval INTERVAL   | Ping interval in seconds           |
| -p,--ping-host PING_HOST | Host for ping requests             |
| -l,--logs LOGS           | Directory for the log files        |
| -w,--web-host WEB_HOST   | Host ip for the webserver          |
| -r,--web-root WEB_ROOT   | Web server root directory          |

------

## TODO

[x] Gzip compression
[x] Include bundled html directly in server binary
