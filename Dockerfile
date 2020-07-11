FROM rust:alpine as builder

RUN apk add -U --no-cache musl-dev

WORKDIR /usr/src/actix

COPY Cargo.toml Cargo.toml

RUN mkdir src/
RUN echo "fn main() { println!(\"Build broke\\nUnderstandable, have a nice day\") }" > src/main.rs
RUN cargo build --release

RUN rm -f target/release/deps/actix-*

COPY . .

RUN cargo build --release

FROM alpine:latest as runner

COPY --from=builder /usr/src/actix/target/release/actix /usr/local/bin/actix

CMD ["actix"]
