# ---- build Yew (WASM) ----
FROM rust:1.85 AS yew-build
RUN apt-get update && apt-get install -y curl ca-certificates && \
    (command -v /usr/local/cargo/bin/rustup >/dev/null || (curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.85.0)) && \
    . /usr/local/cargo/env && \
    /usr/local/cargo/bin/rustup target add wasm32-unknown-unknown && \
    /usr/local/cargo/bin/cargo install trunk --locked
WORKDIR /src
COPY dashboard-spa/ ./dashboard-spa/
COPY assets/ ./assets/
WORKDIR /src/dashboard-spa
RUN . /usr/local/cargo/env && /usr/local/cargo/bin/trunk build --release
# -> écrit dans ../assets/dashboard si Trunk.toml le configure ainsi

# ---- build API ----
FROM rust:1.85 AS api-build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs
RUN cargo build --release
COPY . .
RUN cargo build --release --bin rustfolio

# ---- runtime ----
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=api-build /src/target/release/rustfolio /app/server
COPY assets /app/assets
COPY --from=yew-build /src/assets/dashboard /app/assets/dashboard
COPY templates /app/templates
EXPOSE 8080
USER 65532:65532
ENTRYPOINT ["/app/server"]
