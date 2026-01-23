.PHONY: dev frontend backend db wasm build clean help
.PHONY: docker docker-up docker-down docker-build docker-logs
.PHONY: prod prod-up prod-down prod-build prod-logs
.PHONY: fmt lint test check

# Default target
.DEFAULT_GOAL := help

# ============================================================================
# Development (local)
# ============================================================================

dev: ## Run frontend + backend locally
	bun run dev

frontend: ## Run frontend only
	bun run dev:frontend

backend: ## Run backend only
	bun run dev:backend

backend-watch: ## Run backend with hot reload (requires cargo-watch)
	cd backend && cargo watch -x run

# ============================================================================
# Database
# ============================================================================

db: ## Start PostgreSQL container
	docker-compose up -d postgres

db-ui: ## Start pgAdmin UI
	docker-compose up -d pgadmin

db-down: ## Stop PostgreSQL container
	docker-compose down

db-logs: ## Show PostgreSQL logs
	docker-compose logs -f postgres

# ============================================================================
# WASM
# ============================================================================

wasm: ## Build WASM module
	bun run wasm:build

wasm-watch: ## Build WASM and watch for changes
	cd wasm && cargo watch -s 'wasm-pack build --target web --out-dir ../frontend/src/lib/wasm/pkg'

# ============================================================================
# Build
# ============================================================================

build: ## Build everything (wasm + frontend + backend)
	bun run build

build-frontend: ## Build frontend only
	bun run build:frontend

build-backend: ## Build backend only (release)
	bun run build:backend

# ============================================================================
# Docker - Development (database only)
# ============================================================================

docker: docker-up ## Alias for docker-up

docker-up: ## Start PostgreSQL only
	docker-compose up -d postgres

docker-down: ## Stop all containers
	docker-compose --profile production down

docker-logs: ## Show PostgreSQL logs
	docker-compose logs -f postgres

docker-ps: ## Show running containers
	docker-compose ps

# ============================================================================
# Production (full stack in Docker)
# ============================================================================

prod: prod-up ## Alias for prod-up

prod-up: ## Start full stack (app + postgres) in Docker
	docker-compose --profile production up -d

prod-down: ## Stop full stack
	docker-compose --profile production down

prod-build: ## Build and start full stack
	docker-compose --profile production up -d --build

prod-logs: ## Show all logs
	docker-compose --profile production logs -f

prod-rebuild: ## Force rebuild and start
	docker-compose --profile production build --no-cache
	docker-compose --profile production up -d

# ============================================================================
# Quality
# ============================================================================

fmt: ## Format all code
	bun run fmt

fmt-check: ## Check formatting
	bun run fmt:check

lint: ## Lint frontend (oxlint) + backend (clippy)
	bun run lint
	cargo clippy --workspace

test: ## Run tests
	bun run test:run

check: ## TypeScript/Svelte type check
	bun run check

# ============================================================================
# Setup
# ============================================================================

install: ## Install all dependencies
	bun install
	cd frontend && bun install

setup: install db wasm ## Full setup: install deps, start db, build wasm
	@echo "Setup complete! Run 'make dev' to start development."

# ============================================================================
# Clean
# ============================================================================

clean: ## Clean build artifacts
	rm -rf target
	rm -rf frontend/.svelte-kit
	rm -rf frontend/build
	rm -rf frontend/src/lib/wasm/pkg

clean-all: clean ## Clean everything including node_modules
	rm -rf node_modules
	rm -rf frontend/node_modules

clean-docker: ## Remove Docker volumes and images
	docker-compose --profile production down -v --rmi local

# ============================================================================
# Help
# ============================================================================

help: ## Show this help
	@echo "Usage: make [target]"
	@echo ""
	@echo "Development:"
	@grep -E '^(dev|frontend|backend|backend-simple):.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Database:"
	@grep -E '^db[^:]*:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Production (Docker):"
	@grep -E '^prod[^:]*:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Build:"
	@grep -E '^(build|wasm)[^:]*:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Quality:"
	@grep -E '^(fmt|lint|test|check)[^:]*:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Other:"
	@grep -E '^(install|setup|clean|docker)[^:]*:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
