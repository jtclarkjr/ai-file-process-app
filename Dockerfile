# ============================================================================
# Stage 1: Build Frontend (SvelteKit static output)
# ============================================================================
FROM oven/bun:1 AS frontend-builder

WORKDIR /app

# Copy package files
COPY frontend/package.json frontend/bun.lock* ./

# Install dependencies
RUN bun install --frozen-lockfile

# Copy frontend source
COPY frontend ./

# Build frontend (static adapter produces output in /build)
RUN bun run build

# ============================================================================
# Stage 2: Build Backend (Rust/Axum server)
# ============================================================================
FROM rust:1.83-slim-bookworm AS backend-builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY backend ./backend

# Build release binary
RUN cargo build --release --package backend

# ============================================================================
# Stage 3: Production Runtime
# ============================================================================
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -s /bin/false appuser

WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /app/target/release/backend ./backend

# Copy frontend build (static files served by backend)
COPY --from=frontend-builder /app/build ./static

# Set ownership
RUN chown -R appuser:appuser /app

USER appuser

# Environment variables
ENV RUST_LOG=backend=info,tower_http=info
ENV HOST=0.0.0.0
ENV PORT=8080

EXPOSE 8080

CMD ["./backend"]
