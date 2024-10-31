FROM lukemathwalker/cargo-chef:latest-rust-1.82.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN RUST_BACKTRACE=1 cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin sol 

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install --yes \
    libcurl4 \
    ca-certificates \
    && apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
COPY --from=builder /app/target/release/sol /usr/local/bin
RUN update-ca-certificates
ENTRYPOINT ["/usr/local/bin/sol"]
