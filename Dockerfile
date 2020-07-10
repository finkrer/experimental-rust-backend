FROM rust:alpine as builder

RUN apk add -U --no-cache musl-dev

WORKDIR /usr/src/actix

COPY add_dep.sh add_dep.sh
RUN chmod +x add_dep.sh

COPY Cargo.base.toml Cargo.toml

RUN mkdir src/
RUN echo "fn main() { println!(\"Build broke\\nUnderstandable, have a nice day\") }" > src/main.rs
RUN cargo build --release

RUN ./add_dep.sh actix-files 0.2.2
RUN cargo build --release

RUN ./add_dep.sh actix-web-middleware-redirect-scheme 2.3.3
RUN cargo build --release

RUN rm -f target/release/deps/actix-*

COPY . .

RUN cargo build --release

FROM alpine:latest as runner

COPY --from=builder /usr/src/actix/target/release/actix /usr/local/bin/actix

CMD ["actix"]
