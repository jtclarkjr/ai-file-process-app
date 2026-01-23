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
# Stage 2: Build Backend (install dependencies)
# ============================================================================
FROM oven/bun:1 AS backend-builder

WORKDIR /app

# Copy package files
COPY backend/package.json backend/bun.lock* ./

# Install dependencies
RUN bun install --frozen-lockfile --production

# ============================================================================
# Stage 3: Production Runtime
# ============================================================================
FROM oven/bun:1-slim AS runtime

WORKDIR /app

# Copy backend dependencies and source
COPY --from=backend-builder /app/node_modules ./node_modules
COPY backend/package.json ./
COPY backend/tsconfig.json ./
COPY backend/src ./src

# Copy frontend build (static files served by backend)
COPY --from=frontend-builder /app/build ./static

# Environment variables
ENV LOG_LEVEL=info
ENV HOST=0.0.0.0
ENV PORT=8080

EXPOSE 8080

# Run with Bun
CMD ["bun", "run", "start"]
