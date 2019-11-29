# Ping Log

Simple RESTful webserver for logging and visualizing access times to a specified
host.
It is designed for a raspberry pi or other unix based IoT device running
permanently inside the network.

It is build using [Rust](https://www.rust-lang.org/) and the
[hyper](https://github.com/hyperium/hyper) framework.

## Setup

First [Rust](https://www.rust-lang.org/learn/get-started) has to be installed.

**Build & Execute:**

```bash
cargo run
```

> Cross-compiling is especially easy using [cross](https://github.com/rust-embedded/cross)
>
> E.g. for the raspberry pi: `cross build --target armv7-unknown-linux-gnueabihf`

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
