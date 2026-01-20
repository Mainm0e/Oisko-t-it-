# Build Stage
FROM rust:1.81-slim AS builder
# 1. Install system dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl wget unzip

# 2. Install Dioxus CLI via Cargo (ensures glibc compatibility)
RUN cargo install dioxus-cli --version 0.7.2 --locked

WORKDIR /app
COPY . .
ARG API_URL
ENV API_URL=$API_URL
# Build the WASM and assets
RUN dx build --release --platform web

# Serve Stage (Using Nginx)
FROM nginx:stable-alpine
# Dioxus 0.7 builds to the 'dist' folder by default
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]