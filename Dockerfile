FROM rust:1.52 as builder

WORKDIR /usr/src
COPY Cargo.lock .
COPY Cargo.toml .
COPY src/ src/
RUN cargo build --release
EXPOSE 2137

CMD ["target/release/pastebin"]
