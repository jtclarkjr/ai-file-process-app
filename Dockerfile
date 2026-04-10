# ============================================================================
# Stage 1: Build Frontend (SvelteKit static output)
# ============================================================================
FROM oven/bun:1 AS frontend-builder

WORKDIR /app/frontend

# Install frontend dependencies
COPY frontend/package.json frontend/bun.lock* ./
RUN bun install --frozen-lockfile

# Build static frontend
COPY frontend ./
RUN bun run build

# ============================================================================
# Stage 2: Build Rust Backend
# ============================================================================
FROM rust:1.83-slim-bookworm AS backend-builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Build the real application
COPY Cargo.toml Cargo.lock ./
COPY backend backend
RUN cargo build --release -p backend

# ============================================================================
# Stage 3: Runtime
# ============================================================================
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -s /usr/sbin/nologin appuser

WORKDIR /app

# Copy release binary and static frontend assets
COPY --from=backend-builder /app/target/release/backend /usr/local/bin/backend
COPY --from=frontend-builder /app/frontend/build ./static

RUN chown -R appuser:appuser /app

USER appuser

ENV HOST=0.0.0.0
ENV PORT=8080
ENV RUST_LOG=backend=info,tower_http=info

EXPOSE 8080

CMD ["backend"]
