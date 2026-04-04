FROM rust:1-bookworm AS builder
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
# Build deps layer (empty src to cache dependencies)
RUN mkdir src && echo 'fn main() {}' > src/main.rs && cargo build --release && rm -rf src
COPY src/ src/
# Touch main.rs to force rebuild
RUN touch src/main.rs && cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /build/target/release/verisure-exporter /
EXPOSE 9878
ENTRYPOINT ["/verisure-exporter"]
