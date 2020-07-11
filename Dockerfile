# syntax = docker/dockerfile:experimental

FROM rust:alpine

RUN apk add -U --no-cache musl-dev openssl openssl-dev libc6-compat
RUN ln -s /lib64/ld-linux-x86-64.so.2 /lib/ld64.so.1

WORKDIR /usr/src/actix

COPY . .

RUN --mount=type=cache,mode=0777,target=/usr/src/actix/target cargo build --release; cp /usr/src/actix/target/release/actix /bin

CMD ["actix"]