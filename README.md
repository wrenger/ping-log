# Ping Log

Simple RESTful webserver for logging and visualizing access times to a specified
host.
It is designed for a Raspberry Pi or other Unix-based IoT device running
permanently inside the network.

It is built using [Rust](https://www.rust-lang.org/) and the
[actix](https://actix.rs/) framework.

## Build

Install [yarn](https://yarnpkg.com/getting-started/install).

**Building the front end:**

```
cd ping-view
yarn install
yarn build
```

Install [Rust](https://www.rust-lang.org/learn/get-started).

**Build & execute the server:**

```bash
cargo build
```

> For cross compilation, I would recommend [cross](https://github.com/cross-rs/cross).

## Development

Development on the frontend can be done using `yarn start`.
This automatically reloads the webapp on every change.

> Note that the **server has to be started on port 5000** (`cargo run -- -w 127.0.0.1:5000`) for the yarn proxy to find it.

## Deploy

After building the frontend and server, both the server binary (`target/release/ping-log`)
and the build directory for the webapp (`ping-view/build`) have to be deployed.

If the location of the frontend build directory is different on the target system,
use the `--web` argument of the server to configure it.


**CLI arguments:**

```bash
# with cargo
cargo run -r -- <args>
# or after building & deployment
<path/to>/ping-log <args>
```

The commandline arguments are:

| Argument                 | Description                        |
|--------------------------|------------------------------------|
| -h,--help                | Show this help message and exit    |
| -i,--interval INTERVAL   | Ping interval in seconds           |
| -p,--ping-host PING_HOST | Host for ping requests             |
| -l,--logs LOGS           | Directory for the log files        |
| -w,--web-host WEB_HOST   | Host ip for the webserver          |
| --web DIR                | Web server root directory          |
