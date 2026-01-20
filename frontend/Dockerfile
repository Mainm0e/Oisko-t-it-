# Build Stage
FROM ubuntu:24.04 AS builder

# 1. Install system dependencies
# We need curl and wget to install Rust and the Dioxus CLI
# pkg-config, libssl-dev, and build-essential are needed for original Rust builds
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    pkg-config \
    libssl-dev \
    build-essential \
    unzip \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 2. Install Rust (since we are not using the official rust image)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# 3. Download pre-compiled Dioxus CLI (v0.7.2)
# Ubuntu 24.04 has glibc 2.39, which satisfies the requirement for the binary
RUN wget https://github.com/DioxusLabs/dioxus/releases/download/v0.7.2/dx-x86_64-unknown-linux-gnu.tar.gz \
    && tar -xvf dx-x86_64-unknown-linux-gnu.tar.gz \
    && mv dx /usr/local/bin/ \
    && chmod +x /usr/local/bin/dx

WORKDIR /app
COPY . .

# Set up build arguments for production
ARG API_URL
ENV API_URL=$API_URL

# Debug: Show files and handle context
RUN ls -la
RUN if [ -f "Cargo.toml" ]; then \
    echo "Found Cargo.toml in current directory"; \
    elif [ -d "frontend" ] && [ -f "frontend/Cargo.toml" ]; then \
    echo "Found frontend directory, moving there"; \
    mv frontend/* . && mv frontend/.* . 2>/dev/null || true; \
    else \
    echo "ERROR: Could not find Cargo.toml anywhere!"; \
    ls -R; \
    exit 1; \
    fi

# Build the WASM and assets
RUN dx build --release --platform web

# Serve Stage (Using Nginx)
FROM nginx:stable-alpine
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]