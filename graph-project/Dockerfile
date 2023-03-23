FROM rust:1.68.0-slim as builder
WORKDIR /build
COPY . .
RUN rustup default nightly
RUN apt update && apt install pkg-config libssl-dev -y
RUN cargo install --path .
RUN cargo install refinery_cli

FROM debian:11-slim
ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=postgres
ENV POSTGRES_HOST=localhost
ENV POSTGRES_PORT=5432
ENV POSTGRES_DATABASE=postgres
COPY --from=builder /usr/local/cargo/bin/graph-server /usr/local/bin/graph-server
COPY --from=builder /build/crates/db/migrations /migrations
COPY --from=builder /usr/local/cargo/bin/refinery /usr/local/bin/refinery
COPY start.sh /start.sh
CMD [ "sh", "/start.sh" ]
