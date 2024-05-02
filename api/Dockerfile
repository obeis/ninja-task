FROM rustlang/rust:nightly as builder

COPY . .

RUN cargo build --bin api --release

FROM debian:latest

COPY --from=builder ./target/release/api /usr/bin/

RUN chmod +x /usr/bin/api

Run apt update && apt install -y openssl ca-certificates

EXPOSE 4000

CMD api
