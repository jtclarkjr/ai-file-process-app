# AI File Processing App

AI-powered file processing with a SvelteKit frontend and a Rust backend. Users can upload documents, images, or code files and run operations such as summarize, extract, analyze, classify, or a custom prompt against OpenAI or Anthropic.

## Tech Stack

### Frontend

- **SvelteKit 2** for the app shell and routing
- **Svelte 5** for UI state
- **Vite 8** for local development and builds
- **TypeScript** for type safety

### Backend

- **Rust** for the backend implementation
- **Axum 0.7** for the HTTP API
- **Tokio** for async runtime
- **tower-http** for CORS, tracing, request limits, and static file serving
- **reqwest** for OpenAI and Anthropic API calls
- **infer**, **pdf-extract**, and **docx-rs** for file detection and text extraction

### Tooling

- **Bun** for JavaScript package management and root scripts
- **Cargo** for Rust builds and tests
- **cargo-watch** for backend hot reload during development
- **oxlint**, **dprint**, **Prettier**, and **clippy** for linting and formatting

## Project Structure

```text
.
├── frontend/                # SvelteKit frontend
│   ├── src/
│   │   ├── lib/
│   │   └── routes/
│   ├── package.json
│   └── vite.config.ts
├── backend/                 # Rust API server
│   ├── src/
│   │   ├── main.rs
│   │   ├── errors.rs
│   │   ├── models/
│   │   ├── routes/
│   │   └── services/
│   └── Cargo.toml
├── .env.example
├── Cargo.toml               # Workspace config
├── Makefile
├── Dockerfile
└── docker-compose.yml
```

## Prerequisites

### Required

- **Bun**
- **Rust stable toolchain** with `cargo`

### Recommended

- **cargo-watch** for backend hot reload

```bash
cargo install cargo-watch
```

## Quick Start

```bash
# 1. Clone and enter the project
git clone <repo-url> && cd ai-file-process-app

# 2. Install dependencies
make install

# 3. Create local environment file
cp .env.example .env

# 4. Add at least one provider API key to .env
# OPENAI_API_KEY=...
# or ANTHROPIC_API_KEY=...

# 5. Start frontend + backend
make dev
```

This starts:

- Frontend at `http://localhost:5173`
- Backend API at `http://localhost:8080/api`
- Health check at `http://localhost:8080/api/health`

The frontend redirects `/` to `/process` and talks to the backend on port `8080`.

## Manual Development

Run each side separately if needed:

```bash
# Terminal 1
cd frontend && bun run dev

# Terminal 2
cd backend && cargo watch -x run
```

If you do not have `cargo-watch` installed:

```bash
cd backend && cargo run
```

## Environment Variables

The default local configuration is in [`.env.example`](/Users/jamesclark/GitHub/ai-file-process-app/.env.example).

```bash
# Optional Rust logging
RUST_LOG=backend=debug,tower_http=debug

# Frontend -> backend base URL
VITE_API_BASE=http://localhost:8080

# AI providers
OPENAI_API_KEY=sk-...
OPENAI_MODEL=gpt-4-turbo

ANTHROPIC_API_KEY=sk-ant-...
ANTHROPIC_MODEL=claude-3-sonnet-20240229

# Limits
MAX_FILE_SIZE_MB=10
REQUEST_TIMEOUT_SECS=120
AI_TIMEOUT_SECS=60
```

At least one AI provider key must be configured for file processing to work.

## Supported Operations

- `summarize`
- `extract`
- `analyze`
- `classify`
- `custom`

## Supported File Types

- PDF
- DOCX
- Plain text
- Markdown
- Source code files such as `py`, `js`, `ts`, `tsx`, `rs`, `go`, `java`, `json`, `yaml`, `html`, `css`, `sql`, `toml`
- Images: `jpg`, `jpeg`, `png`, `gif`, `webp`

The backend validates file type using magic bytes when possible and falls back to extension or declared MIME type where needed.

## API Endpoints

| Method | Endpoint                     | Description                |
| ------ | ---------------------------- | -------------------------- |
| GET    | `/api/health`                | Health check               |
| POST   | `/api/files/process`         | Process an uploaded file   |
| GET    | `/api/files/supported-types` | List accepted file types   |
| GET    | `/api/files/providers`       | List configured providers  |

## Common Commands

Run `make help` for the full list.

### Development

| Command         | Description                              |
| --------------- | ---------------------------------------- |
| `make dev`      | Start frontend and backend               |
| `make frontend` | Start frontend only                      |
| `make backend`  | Start backend only with hot reload       |

### Quality

| Command          | Description                          |
| ---------------- | ------------------------------------ |
| `make fmt`       | Format frontend and backend          |
| `make fmt-check` | Check formatting                     |
| `make lint`      | Run oxlint and clippy                |
| `make test`      | Run frontend tests and `cargo test`  |
| `make check`     | Run Svelte and TypeScript checks     |

### Setup and Cleanup

| Command          | Description                                |
| ---------------- | ------------------------------------------ |
| `make install`   | Install Bun packages and build the backend |
| `make setup`     | Alias for full setup                       |
| `make clean`     | Remove build artifacts                     |
| `make clean-all` | Remove build artifacts and dependencies    |

## Architecture

```text
┌─────────────────────────────┐
│      SvelteKit Frontend     │
│  - Upload UI                │
│  - Operation selection      │
│  - Result display           │
└──────────────┬──────────────┘
               │ HTTP
               ▼
┌─────────────────────────────┐
│       Rust API (Axum)       │
│  - Multipart upload         │
│  - File validation          │
│  - Text/image extraction    │
│  - Provider dispatch        │
└──────────────┬──────────────┘
               │
               ▼
┌─────────────────────────────┐
│ OpenAI / Anthropic APIs     │
└─────────────────────────────┘
```

## Notes

- The Rust server can serve static files from `./static` when that directory exists.
- The root `Dockerfile` and `docker-compose.yml` still reflect an older Python-based deployment path and need to be updated separately to match the current Rust backend.

## License

MIT
