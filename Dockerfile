FROM rust:1.49 as builder

WORKDIR /usr/src

RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

EXPOSE 3030

RUN USER=root cargo new crawler
WORKDIR /usr/src/crawler
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/crawler .
CMD ["./crawler"]
