.PHONY: dev frontend backend build clean help
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

backend: ## Run backend only (uvicorn with hot reload)
	cd backend && uvicorn app.main:app --reload --host 0.0.0.0 --port 8080

# ============================================================================
# Build
# ============================================================================

build: build-frontend ## Build everything (frontend)
	@echo "Build complete!"

build-frontend: ## Build frontend only
	cd frontend && bun run build

# ============================================================================
# Docker - Development (backend with hot-reload)
# ============================================================================

docker-dev: ## Start backend in Docker with hot-reload
	docker-compose --profile dev up

docker-dev-build: ## Build and start dev backend
	docker-compose --profile dev up --build

docker-dev-down: ## Stop dev backend
	docker-compose --profile dev down

docker-dev-logs: ## Show dev backend logs
	docker-compose --profile dev logs -f backend-dev

docker-dev-shell: ## Shell into running dev container
	docker-compose --profile dev exec backend-dev bash

docker-dev-run: ## Run a one-off dev container with shell
	docker-compose --profile dev run --rm -it backend-dev bash

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
	cd backend && ruff format .

fmt-check: ## Check formatting
	bun run fmt:check
	cd backend && ruff format --check .

lint: ## Lint frontend (oxlint) + backend (ruff)
	bun run lint
	cd backend && ruff check .

test: ## Run tests
	bun run test:run
	cd backend && pytest tests/ -v

check: ## TypeScript/Svelte type check
	bun run check

# ============================================================================
# Setup
# ============================================================================

install: ## Install all dependencies
	bun install
	cd frontend && bun install
	cd backend && pip install -e ".[dev]"

setup: install ## Full setup: install deps
	@echo "Setup complete! Run 'make dev' to start development."

# ============================================================================
# Clean
# ============================================================================

clean: ## Clean build artifacts
	rm -rf frontend/.svelte-kit
	rm -rf frontend/build
	rm -rf backend/__pycache__
	rm -rf backend/app/__pycache__
	rm -rf backend/.pytest_cache

clean-all: clean ## Clean everything including node_modules
	rm -rf node_modules
	rm -rf frontend/node_modules
	rm -rf backend/.venv

clean-docker: ## Remove Docker volumes and images
	docker-compose --profile dev --profile production down -v --rmi local

# ============================================================================
# Help
# ============================================================================

help: ## Show this help
	@echo "Usage: make [target]"
	@echo ""
	@echo "Development (Local):"
	@grep -E '^(dev|frontend|backend):.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Development (Docker):"
	@grep -E '^docker-dev[^:]*:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Production (Docker):"
	@grep -E '^prod[^:]*:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Build:"
	@grep -E '^build[^:]*:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Quality:"
	@grep -E '^(fmt|lint|test|check)[^:]*:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Other:"
	@grep -E '^(install|setup|clean)[^:]*:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2}'
