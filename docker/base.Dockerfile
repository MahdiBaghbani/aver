FROM rust:1.81.0-bookworm@sha256:d5737b7651a46cdd3c42155a5a87ad21945e5e86a0e4a1cc76a7e00026feca7e AS build

WORKDIR /aver

COPY ./.cargo                           ./.cargo
COPY ./crates                           ./crates
COPY ./Cargo.toml                       ./Cargo.toml
COPY ./Cargo.lock                       ./Cargo.lock

RUN cargo build --release --package aver --bin aver

FROM gcr.io/distroless/cc-debian12@sha256:1850aee2ff72864350058d83d681c757d45c885986d15fcca7309b9e5c69f39a

ARG BUILD_DATE
ARG VERSION
ARG REVISION

# keys for oci taken from:
# https://github.com/opencontainers/image-spec/blob/main/annotations.md#pre-defined-annotation-keys
LABEL org.opencontainers.image.title="Aver"
LABEL org.opencontainers.image.description="Reva but backwards"
LABEL org.opencontainers.image.vendor="Azadeh Afzar"
LABEL org.opencontainers.image.licenses="AGPL-3.0-only"
LABEL org.opencontainers.image.url="https://git.azadehafzar.io/mahdi-baghbani/aver"
LABEL org.opencontainers.image.source="https://git.azadehafzar.io/mahdi-baghbani/aver"
LABEL org.opencontainers.image.documentation="https://git.azadehafzar.io/mahdi-baghbani/aver"
LABEL org.opencontainers.image.authors="Mohammad Mahdi Baghbani Pourvahid <mahdi-baghbani@azadehafzar.io>"
LABEL org.opencontainers.image.created="${BUILD_DATE-'not specified'}"
LABEL org.opencontainers.image.version="${VERSION-'test'}"
LABEL org.opencontainers.image.revision="${REVISION-'test'}"

ENV PATH="/binaries:${PATH}"

WORKDIR /binaries

COPY --from=build /aver/target/release/aver .
