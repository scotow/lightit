FROM rust:1.58-slim AS builder

RUN apt-get update && apt-get install -y openssl libssl-dev pkg-config

WORKDIR /app
COPY . .
RUN cargo build --package lightit-cli --release 

#------------

FROM debian:bullseye-slim

COPY --from=builder /app/target/release/lightit-cli /lightit

ENTRYPOINT ["/lightit"]
