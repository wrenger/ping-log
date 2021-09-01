# Ping Log

Simple RESTful webserver for logging and visualizing access times to a specified
host.
It is designed for a raspberry pi or other unix based IoT device running
permanently inside the network.

It is build using [Rust](https://www.rust-lang.org/) and the
[actix](https://actix.rs/) framework.

# Usage

The latest release for Raspberry Pi 4 and newer can be downloaded [here](https://gitlab.com/wrenger/rust-ping-log/-/releases).

This can be executed with:

```bash
# after unzipping and navigating into the diretory:
./ping-<version>-aarch64
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

## Build

First [Rust](https://www.rust-lang.org/learn/get-started) has to be installed.

**Build & Execute:**

```bash
cargo run
```

> For cross compiling with docker see down below.

**Optional arguments:**

```bash
cargo run -- <args>
```

> The commandline arguments (`<args>`) are the same as above.

## Docker CI Cross-Compilation

For the 64bit Raspberry Pi 4 and newer.

```bash
# Build the docker image (only once)
docker build -t registry.gitlab.com/wrenger/rust-ping-log docker
# Upload to GitLab CI (optional)
docker push registry.gitlab.com/wrenger/rust-ping-log
# Execute locally
docker run --rm -it --user "$(id -u)":"$(id -g)" \
    --volume=$(pwd):/home/docker/project -w /home/docker/project \
    registry.gitlab.com/wrenger/rust-ping-log \
    cargo build --target=aarch64-unknown-linux-gnu --release
```
