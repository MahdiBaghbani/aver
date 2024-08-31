FROM rust:1.80.1-bookworm@sha256:29fe4376919e25b7587a1063d7b521d9db735fc137d3cf30ae41eb326d209471 AS build

WORKDIR /aver

COPY ./.cargo                           ./.cargo
COPY ./crates                           ./crates
COPY ./Cargo.toml                       ./Cargo.toml
COPY ./Cargo.lock                       ./Cargo.lock

RUN cargo build --release --package aver --bin aver

FROM debian:bookworm-slim@sha256:2ccc7e39b0a6f504d252f807da1fc4b5bcd838e83e4dec3e2f57b2a4a64e7214

ENV PATH="/binaries:${PATH}"

WORKDIR /binaries

COPY --from=build /aver/target/release/aver .
