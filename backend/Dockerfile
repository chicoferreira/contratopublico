FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /backend

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /backend/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin backend

FROM debian:bookworm-slim AS runtime
WORKDIR /backend
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /backend/target/release/backend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/backend"]
