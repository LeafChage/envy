FROM rust:1.71 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

ENTRYPOINT ["/app/target/release/envy"]
