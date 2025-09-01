FROM rust:bookworm AS builder

WORKDIR /build

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

# FIXME: Apparently installing cURL makes reqwest TLS work
# Will fix this at a latter time
RUN apt-get update && apt install -y openssl curl

COPY --from=builder /build/target/release/install-sh install-sh

CMD ["./install-sh"]