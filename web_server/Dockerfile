FROM rustlang/rust:nightly as builder

COPY . .

RUN cargo build --bin web_server --release
RUN cargo install grass
RUN grass web/assets/sass/main.scss web/assets/main.css
RUN cargo install dioxus-cli@0.5.0
RUN cd web && dx build --release

FROM debian:latest

COPY --from=builder ./target/release/web_server /usr/bin/

COPY --from=builder ./web/dist /var/web

RUN chmod +x /usr/bin/web_server

EXPOSE 8080

CMD web_server
