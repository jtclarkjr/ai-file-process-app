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
# Stage 2: Python Backend Runtime
# ============================================================================
FROM python:3.12-slim-bookworm AS runtime

# Install system dependencies (libmagic for python-magic)
RUN apt-get update && apt-get install -y --no-install-recommends \
    libmagic1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -s /bin/false appuser

WORKDIR /app

# Install Python dependencies
COPY backend/pyproject.toml ./
RUN pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir .

# Copy application code
COPY backend/app ./app

# Copy frontend build (static files served by backend)
COPY --from=frontend-builder /app/build ./static

# Set ownership
RUN chown -R appuser:appuser /app

USER appuser

# Environment variables
ENV PYTHONUNBUFFERED=1
ENV LOG_LEVEL=INFO
ENV HOST=0.0.0.0
ENV PORT=8080

EXPOSE 8080

# Run with uvicorn
CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8080"]
