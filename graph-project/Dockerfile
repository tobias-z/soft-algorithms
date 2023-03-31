FROM rust:1.68.0-slim as builder
WORKDIR /build
COPY . .
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN apt update && apt install pkg-config libssl-dev -y
RUN cargo install cargo-leptos
RUN cargo leptos build --release
RUN cargo install refinery_cli

FROM debian:11-slim
RUN mkdir -p /opt/graph-project/target/site
COPY --from=builder /build/target/server/release/graph-project /opt/graph-project/graph-project
COPY --from=builder /build/target/site /opt/graph-project/target/site
COPY --from=builder /build/crates/db/migrations /migrations
COPY --from=builder /usr/local/cargo/bin/refinery /usr/local/bin/refinery
COPY start.sh /start.sh
CMD [ "sh", "/start.sh" ]
