# Builds and tests the plugin without installing Rust locally.
# `docker build --output dist .` writes a ready-to-install plugin folder to dist/.
FROM rust:1.99.1-slim-bookworm AS build
RUN rustup target add wasm32-unknown-unknown
WORKDIR /plugin
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo test && cargo build --release --target wasm32-unknown-unknown

FROM scratch
COPY --from=build /plugin/target/wasm32-unknown-unknown/release/scpcb_plugin.wasm /plugin.wasm
COPY metadata.json icon.png /
