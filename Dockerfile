FROM rust:latest

WORKDIR /usr/src/actix

COPY Cargo.toml Cargo.toml

RUN mkdir src/
RUN echo "fn main() { println!(\"Build broke\\nUnderstandable, have a nice day\") }" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/actix-*

COPY . .

RUN cargo build --release
RUN cargo install --path .

CMD ["/usr/local/cargo/bin/actix"]
