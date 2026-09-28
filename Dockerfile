FROM rust:1.90-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home picker
WORKDIR /app
COPY --from=builder /app/target/release/team-event-picker /usr/local/bin/team-event-picker
COPY src/assets ./src/assets
USER picker
EXPOSE 8080
CMD ["team-event-picker"]
