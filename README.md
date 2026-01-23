# AI File Processing App

A fullstack application for processing files with AI providers (OpenAI, Anthropic). Built with SvelteKit (frontend) and FastAPI (backend).

## Tech Stack

### Frontend

- **SvelteKit 2.50+** - Full-stack Svelte framework with static adapter
- **Svelte 5** - With runes for state management
- **Vite 8** - Rust-based bundler for fast builds
- **TypeScript** - Type safety

### Backend

- **FastAPI** - Modern Python async web framework
- **Pydantic** - Data validation and settings management
- **OpenAI/Anthropic SDKs** - Official AI provider clients
- **python-magic** - MIME type detection
- **pypdf** - PDF text extraction
- **python-docx** - DOCX parsing

### Tooling

- **Bun** - JavaScript runtime & package manager
- **Docker** - Container deployment
- **oxlint** - Rust-based linter
- **Prettier** - Code formatting
- **pytest** - Python testing

## Project Structure

```
├── frontend/           # SvelteKit static app
│   ├── src/
│   │   ├── lib/        # Reusable components
│   │   └── routes/     # SvelteKit routes
│   ├── vite.config.ts
│   └── package.json
│
├── backend/            # FastAPI REST API
│   ├── app/
│   │   ├── main.py     # FastAPI app entry
│   │   ├── config.py   # Environment config
│   │   ├── exceptions.py
│   │   ├── models/     # Pydantic models
│   │   ├── services/   # Business logic
│   │   └── routes/     # API endpoints
│   ├── tests/          # Test suite
│   └── pyproject.toml  # Python dependencies
│
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

# Python 3.12+
# macOS: brew install python@3.12
# Ubuntu: sudo apt install python3.12 python3.12-venv

# libmagic (for file type detection)
# macOS: brew install libmagic
# Ubuntu: sudo apt install libmagic1

# Docker (optional, for containerized deployment)
# Download from https://www.docker.com/products/docker-desktop
```

### Optional

```bash
# Make (for Makefile commands)
brew install make  # macOS
apt-get install make  # Ubuntu/Debian
```

## Quick Start

```bash
# 1. Clone and enter the project
git clone <repo-url> && cd ai-file-process-app

# 2. Install frontend dependencies
cd frontend && bun install && cd ..

# 3. Install backend dependencies
cd backend && pip install -e ".[dev]" && cd ..

# 4. Create .env file with API keys
cp .env.example .env
# Edit .env and add your API keys

# 5. Start development servers
# Terminal 1: Frontend
cd frontend && bun run dev

# Terminal 2: Backend
cd backend && uvicorn app.main:app --reload --port 8080
```

**URLs:**

- Frontend: http://localhost:5173
- Backend API: http://localhost:8080/api
- API Documentation: http://localhost:8080/docs
- The frontend proxies `/api` requests to the backend

## Makefile Commands

Run `make help` to see all available commands.

### Development

| Command         | Description                                  |
| --------------- | -------------------------------------------- |
| `make dev`      | Start frontend + backend concurrently        |
| `make frontend` | Start frontend only (SvelteKit dev server)   |
| `make backend`  | Start backend only (uvicorn with hot reload) |

### Build

| Command               | Description                  |
| --------------------- | ---------------------------- |
| `make build`          | Build everything             |
| `make build-frontend` | Build SvelteKit static files |

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

- Multi-stage build: Frontend → Python Runtime
- Serves SvelteKit static files from the backend
- Single container at http://localhost:8080
- Python 3.12 slim image

**Docker Build Process:**

1. **Stage 1 (Frontend)**: Builds SvelteKit static files using Bun
2. **Stage 2 (Runtime)**: Python 3.12 with uvicorn serving both API + static files

### Manual Build

```bash
# Build frontend
cd frontend && bun run build && cd ..

# Install backend dependencies
cd backend && pip install -e . && cd ..

# Copy frontend build to static directory
cp -r frontend/build backend/static
```

Then run the server:

```bash
export OPENAI_API_KEY=sk-...
export ANTHROPIC_API_KEY=sk-ant-...
cd backend && uvicorn app.main:app --host 0.0.0.0 --port 8080
```

## API Endpoints

| Method | Endpoint                     | Description                     |
| ------ | ---------------------------- | ------------------------------- |
| GET    | `/api/health`                | Health check                    |
| POST   | `/api/files/process`         | Process file with AI            |
| GET    | `/api/files/supported-types` | List supported file types       |
| GET    | `/api/files/providers`       | List available AI providers     |
| GET    | `/docs`                      | OpenAPI documentation (Swagger) |
| GET    | `/redoc`                     | ReDoc documentation             |

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
ANTHROPIC_MODEL=claude-3-sonnet-20240229  # Default: claude-3-sonnet-20240229

# Server settings
HOST=0.0.0.0                       # Server bind address
PORT=8080                          # Server port
LOG_LEVEL=INFO                     # Logging level (DEBUG, INFO, WARNING, ERROR)

# File processing
MAX_FILE_SIZE_MB=10                # Max upload size
AI_TIMEOUT_SECS=60                 # AI provider timeout
REQUEST_TIMEOUT_SECS=120           # Overall request timeout
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
│  │   Python Backend (FastAPI)      │    │
│  │  - Serves API at /api/*         │    │
│  │  - Serves static files at /     │    │
│  │  - OpenAI/Anthropic SDKs        │    │
│  │  - Auto-generated API docs      │    │
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

- **Multi-Provider Support** - OpenAI and Anthropic with official SDKs
- **File Processing** - PDF, DOCX, TXT, images, and code files
- **MIME Validation** - Magic byte detection for security
- **Error Handling** - Graceful error messages with proper HTTP status codes
- **Responsive UI** - Works on desktop and mobile
- **Type Safety** - Full TypeScript frontend + Pydantic models
- **Docker Ready** - One-command deployment
- **API Documentation** - Auto-generated OpenAPI docs at /docs

## Security

- **Non-root Container** - Runs as unprivileged user
- **No Persistent Storage** - Stateless containers
- **Secrets via Environment** - API keys not in image
- **Minimal Dependencies** - Small attack surface
- **Read-only Filesystem** - Can add `--read-only` flag

## License

MIT
