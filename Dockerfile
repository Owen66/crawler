FROM rust:1.49 as builder
WORKDIR /app
RUN USER=root cargo new crawler
WORKDIR /app/crawler
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /srv/crawler
COPY --from=builder /app/crawler/target/release/crawler .
EXPOSE 8080
CMD ["./crawler"]
