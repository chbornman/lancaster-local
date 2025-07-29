FROM rust:latest as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Build dependencies (this layer will be cached)
RUN mkdir -p src/bin && \
    echo "fn main() {}" > src/main.rs && \
    echo "fn main() {}" > src/bin/seed.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY . .

# Build application
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/lancaster-backend /app/lancaster-backend
COPY --from=builder /app/target/release/seed /app/seed
COPY --from=builder /app/migrations /app/migrations

# Create non-root user
RUN useradd -r -s /bin/false appuser && \
    chown -R appuser:appuser /app
USER appuser

EXPOSE 3000

CMD ["./lancaster-backend"]