# Ping Log

Simple RESTful webserver for logging and visualizing access times to a specified
host.
It is designed for a Raspberry Pi or other Unix-based IoT device running
permanently inside the network.

It is built using [Rust](https://www.rust-lang.org) and the
[Dioxus](https://dioxuslabs.com) framework.

## Build

Install [Rust](https://www.rust-lang.org/learn/get-started).

Install the [dioxus-cli](https://crates.io/crates/dioxus-cli):

```sh
cargo install dioxus-cli
```

**Build & execute the server:**

```sh
dx build -r
```

The server and ressources are stored under `target/dx/ping-log/release/web`.

> For cross compilation, I would recommend [cross](https://github.com/cross-rs/cross).
>
> ```sh
> cross build --target aarch64-unknown-linux-musl -r -Fserver
> ```

## Development

This project includes basic organization with an organized `assets` folder and a `src/components` folder.

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```sh
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```sh
dx serve
```

**CLI arguments:**

```sh
# after building & deployment (dx build -r)
./target/dx/ping-log/release/web/server <args>
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
