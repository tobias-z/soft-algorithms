FROM rust:1.68.0-slim as builder
WORKDIR /build
COPY . .
RUN rustup default nightly
RUN apt update && apt install pkg-config libssl-dev -y
RUN cargo build --release
RUN cargo install refinery_cli --no-default-features --features=postgresql

FROM debian:11-slim
COPY --from=builder /build/target/release/graph-project /usr/local/bin/graph-project
COPY --from=builder /build/crates/db/migrations /migrations
COPY --from=builder /usr/local/cargo/bin/refinery /usr/local/bin/refinery
COPY start.sh /start.sh
CMD [ "sh", "/start.sh" ]
