FROM rust:latest as builder

RUN USER=root cargo new --bin rust-poker
WORKDIR /rust-poker

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /rust-poker/target/release/rust-poker /usr/local/bin/rust-poker

EXPOSE 3000

CMD ["rust-poker"]
