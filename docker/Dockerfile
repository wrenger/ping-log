FROM rust:latest

RUN apt-get update && \
    apt-get install --assume-yes gcc-aarch64-linux-gnu

RUN rustup target add aarch64-unknown-linux-gnu
