# Use Rust nightly for Leptos development
FROM rustlang/rust:nightly-bookworm

# Update package list
RUN apt-get update -y

# Install some useful tools/shortcuts
RUN apt-get install -y tree curl wget

# Install required tools for Leptos development
RUN apt-get install -y --no-install-recommends \
    clang \
    pkg-config \
    libssl-dev \
    build-essential \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-binstall for easier cargo extension installation
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && cp cargo-binstall /usr/local/cargo/bin \
    && rm cargo-binstall-x86_64-unknown-linux-musl.tgz cargo-binstall

# Install cargo-leptos using cargo-binstall
RUN cargo binstall cargo-leptos -y

# Add the WASM target for Leptos
RUN rustup target add wasm32-unknown-unknown

# create container user with same rights than host user

RUN groupadd -g 1000 appuser && \
    useradd -m -u 1000 -g 1000 -s /bin/bash appuser

# global configuration done ! let's focus on the project itself

WORKDIR /app
RUN chown -R appuser:appuser /app

USER appuser

WORKDIR /app/crates/services/web