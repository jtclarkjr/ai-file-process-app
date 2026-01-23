# AI File Processing App

A fullstack application for processing files with AI providers (OpenAI, Anthropic). Built with SvelteKit (frontend) and Axum (backend).

## Tech Stack

### Frontend
- **SvelteKit 2.50+** - Full-stack Svelte framework with static adapter
- **Svelte 5** - With runes for state management
- **Vite 8** - Rust-based bundler for fast builds
- **TypeScript** - Type safety

### Backend
- **Axum** - Rust async web framework
- **Tokio** - Async runtime
- **OpenAI/Anthropic APIs** - AI providers for file processing

### Tooling
- **Bun** - JavaScript runtime & package manager
- **Docker** - Container deployment
- **oxlint** - Rust-based linter
- **Prettier** - Code formatting

## Project Structure

```
├── frontend/           # SvelteKit static app
│   ├── src/
│   │   ├── lib/        # Reusable components
│   │   └── routes/     # SvelteKit routes
│   ├── vite.config.ts
│   └── package.json
│
├── backend/            # Axum REST API
│   └── src/
│       ├── main.rs     # Server entry
│       ├── routes/     # API endpoints
│       ├── handlers/   # Request handlers
│       └── models/     # Data structures
│
├── Cargo.toml          # Rust workspace
├── package.json        # Root bun scripts
├── Dockerfile          # Multi-stage build
├── docker-compose.yml  # Docker compose config
└── Makefile            # Development commands
```

## Prerequisites

### Required

```bash
# Bun (JavaScript runtime & package manager)
curl -fsSL https://bun.sh/install | bash

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Docker (optional, for containerized deployment)
# Download from https://www.docker.com/products/docker-desktop
```

### Optional

```bash
# cargo-watch (backend hot reload)
cargo install cargo-watch

# Make (for Makefile commands)
brew install make  # macOS
apt-get install make  # Ubuntu/Debian
```

## Quick Start

```bash
# 1. Clone and enter the project
git clone <repo-url> && cd ai-file-process-app

# 2. Install dependencies
make install

# 3. Create .env file with API keys
cp .env.example .env
# Edit .env and add your API keys

# 4. Start development servers
make dev
```

**URLs:**
- Frontend: http://localhost:5173
- Backend API: http://localhost:8080/api
- The frontend proxies `/api` requests to the backend

## Makefile Commands

Run `make help` to see all available commands.

### Development

| Command              | Description                                       |
| -------------------- | ------------------------------------------------- |
| `make dev`           | Start frontend + backend concurrently             |
| `make frontend`      | Start frontend only (SvelteKit dev server)        |
| `make backend`       | Start backend only                                |
| `make backend-watch` | Start backend with hot reload (needs cargo-watch) |

### Build

| Command               | Description                  |
| --------------------- | ---------------------------- |
| `make build`          | Build everything             |
| `make build-frontend` | Build SvelteKit static files |
| `make build-backend`  | Build backend (release)      |

### Production (Docker)

| Command             | Description             |
| ------------------- | ----------------------- |
| `make prod`         | Start full stack        |
| `make prod-build`   | Build and start         |
| `make prod-logs`    | Show container logs     |
| `make prod-down`    | Stop all containers     |
| `make prod-rebuild` | Force rebuild and start |

### Quality

| Command          | Description                     |
| ---------------- | ------------------------------- |
| `make fmt`       | Format all code                 |
| `make fmt-check` | Check formatting                |
| `make lint`      | Lint frontend + backend         |
| `make test`      | Run tests                       |
| `make check`     | TypeScript/Svelte type checking |

### Setup & Clean

| Command          | Description                         |
| ---------------- | ----------------------------------- |
| `make setup`     | Full setup (install + dependencies) |
| `make install`   | Install all dependencies            |
| `make clean`     | Clean build artifacts               |
| `make clean-all` | Clean everything incl. node_modules |

## Bun Scripts

Alternative to Makefile:

```bash
bun run dev           # Start frontend + backend
bun run dev:frontend  # Start frontend only
bun run dev:backend   # Start backend
bun run build         # Build everything
bun run fmt           # Format code
bun run lint          # Lint code
bun run test          # Run tests
```

## Production Deployment

### Docker (Recommended)

```bash
# Create .env file with API keys
cp .env.example .env
# Edit .env and add your API keys

# Build and run with docker-compose
docker-compose up --build

# Or use Makefile
make prod-build
```

The Docker deployment:
- Multi-stage build: Frontend → Backend → Runtime
- Serves SvelteKit static files from the backend
- Single container at http://localhost:8080
- Minimal Debian image (~150MB)

**Docker Build Process:**
1. **Stage 1 (Frontend)**: Builds SvelteKit static files using Bun
2. **Stage 2 (Backend)**: Compiles Rust backend binary
3. **Stage 3 (Runtime)**: Minimal Debian image with both backend + static files

### Manual Build

```bash
make build

# Outputs:
# - frontend/build/         → Static files (served by backend)
# - target/release/backend  → Backend binary
```

Then run the binary:
```bash
export OPENAI_API_KEY=sk-...
export ANTHROPIC_API_KEY=sk-ant-...
./target/release/backend
```

## API Endpoints

| Method | Endpoint          | Description                |
| ------ | ----------------- | -------------------------- |
| GET    | `/api/health`     | Health check               |
| POST   | `/api/process`    | Process file with AI       |

## Environment Variables

### Required for Running

At least one AI provider API key is required:

```bash
# OpenAI API key
OPENAI_API_KEY=sk-...

# OR Anthropic API key
ANTHROPIC_API_KEY=sk-ant-...
```

### Optional Configuration

```bash
# AI Provider settings
OPENAI_MODEL=gpt-4-turbo           # Default: gpt-4-turbo
ANTHROPIC_MODEL=claude-3-opus      # Default: claude-3-opus-20240229

# Server settings
HOST=0.0.0.0                       # Server bind address
PORT=8080                          # Server port
RUST_LOG=backend=info,tower_http=info

# File processing
MAX_FILE_SIZE_MB=10                # Max upload size
AI_TIMEOUT_SECS=60                 # AI provider timeout
```

### Development vs Production

**Development (.env file in root):**
```bash
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
```

**Production (Docker environment):**
```bash
# docker-compose.yml loads .env automatically
# OR pass as environment variables:
docker run -e OPENAI_API_KEY=sk-... -e ANTHROPIC_API_KEY=sk-ant-... ...
```

## Architecture

```
┌─────────────────────────────────────────┐
│           Docker Container              │
│  ┌─────────────────────────────────┐    │
│  │     Rust Backend (Axum)         │    │
│  │  - Serves API at /api/*         │    │
│  │  - Serves static files at /     │    │
│  │  - OpenAI/Anthropic integration │    │
│  └─────────────────────────────────┘    │
│              ↓ serves                   │
│  ┌─────────────────────────────────┐    │
│  │   Static Frontend (SvelteKit)   │    │
│  │   - Built at /app/static        │    │
│  │   - Client-side routing (SPA)   │    │
│  └─────────────────────────────────┘    │
└─────────────────────────────────────────┘
         ↕ API calls
┌─────────────────────────────────────────┐
│   External AI APIs (OpenAI/Anthropic)   │
└─────────────────────────────────────────┘
```

## Features

- **Multi-Provider Support** - OpenAI and Anthropic
- **File Processing** - PDF, TXT, images, etc.
- **Streaming Responses** - Real-time AI output
- **Error Handling** - Graceful error messages
- **Responsive UI** - Works on desktop and mobile
- **Type Safety** - Full TypeScript + Rust types
- **Docker Ready** - One-command deployment

## Security

- **Non-root Container** - Runs as unprivileged user
- **No Persistent Storage** - Stateless containers
- **Secrets via Environment** - API keys not in image
- **Minimal Dependencies** - Small attack surface
- **Read-only Filesystem** - Can add `--read-only` flag

## License

MIT
