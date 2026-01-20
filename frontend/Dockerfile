# Build Stage
FROM rust:1.81-slim AS builder
# 1. Install system dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl wget unzip

# 2. Download pre-compiled Dioxus CLI (v0.7.2 is the latest stable)
# This replaces the "cargo install" line
RUN wget https://github.com/DioxusLabs/dioxus/releases/download/v0.7.2/dx-x86_64-unknown-linux-gnu.tar.gz \
    && tar -xvf dx-x86_64-unknown-linux-gnu.tar.gz \
    && mv dx /usr/local/bin/ \
    && chmod +x /usr/local/bin/dx

WORKDIR /app
COPY . .
# Build the WASM and assets
RUN dx build --release --platform web

# Serve Stage (Using Nginx)
FROM nginx:stable-alpine
# Dioxus 0.7 builds to the 'dist' folder by default
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]