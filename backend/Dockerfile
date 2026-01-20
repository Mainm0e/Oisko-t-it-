# Build Stage
FROM rust:1.84-slim AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /app
COPY . .
RUN cargo build --release

# Run Stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates
WORKDIR /app
COPY --from=builder /app/target/release/backend .
EXPOSE 3000
CMD ["./backend"]