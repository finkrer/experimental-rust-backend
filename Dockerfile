# syntax = docker/dockerfile:experimental

FROM rust:alpine

RUN apk add -U --no-cache musl-dev

WORKDIR /usr/src/actix

COPY . .

RUN --mount=type=cache,mode=0777,target=/usr/src/actix/target cargo build --release; cp /usr/src/actix/target/release/actix /bin

EXPOSE 8080

CMD ["actix"]