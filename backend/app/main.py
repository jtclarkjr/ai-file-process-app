"""FastAPI application."""

import logging
import sys
from contextlib import asynccontextmanager
from pathlib import Path

from fastapi import FastAPI, Request
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import FileResponse, JSONResponse
from fastapi.staticfiles import StaticFiles

from app.config import settings
from app.exceptions import AppException
from app.routes import files, health
from app.services import AnthropicProvider, FileExtractor, OpenAiProvider

# Configure logging
logging.basicConfig(
    level=settings.log_level,
    format="%(asctime)s - %(name)s - %(levelname)s - %(message)s",
    handlers=[logging.StreamHandler(sys.stdout)],
)
logger = logging.getLogger(__name__)


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Lifespan context manager for app startup and shutdown."""
    # Startup
    logger.info("Starting up AI File Processor backend")

    # Initialize AI providers
    openai = None
    anthropic = None

    if settings.openai_api_key:
        logger.info(f"OpenAI provider configured with model: {settings.openai_model}")
        openai = OpenAiProvider(
            api_key=settings.openai_api_key,
            model=settings.openai_model,
            timeout_secs=settings.ai_timeout_secs,
        )
    else:
        logger.warning("No OpenAI API key configured")

    if settings.anthropic_api_key:
        logger.info(f"Anthropic provider configured with model: {settings.anthropic_model}")
        anthropic = AnthropicProvider(
            api_key=settings.anthropic_api_key,
            model=settings.anthropic_model,
            timeout_secs=settings.ai_timeout_secs,
        )
    else:
        logger.warning("No Anthropic API key configured")

    if openai is None and anthropic is None:
        logger.warning("No AI providers configured. Set OPENAI_API_KEY or ANTHROPIC_API_KEY.")

    # Create file extractor
    extractor = FileExtractor(settings)

    # Set the global providers in the files router
    files.set_providers(openai=openai, anthropic=anthropic, extractor=extractor)

    logger.info(
        f"File processing: max_size={settings.max_file_size_mb}MB, "
        f"request_timeout={settings.request_timeout_secs}s, "
        f"ai_timeout={settings.ai_timeout_secs}s"
    )

    yield

    # Shutdown
    logger.info("Shutting down AI File Processor backend")


# Create FastAPI app
app = FastAPI(
    title="AI File Processor",
    description="Process files with AI providers",
    version="1.0.0",
    lifespan=lifespan,
)


# Exception handler for custom AppException
@app.exception_handler(AppException)
async def app_exception_handler(request: Request, exc: AppException):
    """Handle custom app exceptions."""
    return JSONResponse(
        status_code=exc.status_code,
        content=exc.detail,
    )


# CORS middleware - allow all origins for development
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Request size limit middleware (FastAPI handles this via max_size parameter in Depends)
# We'll use Starlette's built-in limit instead
from starlette.middleware.base import BaseHTTPMiddleware


class RequestSizeLimitMiddleware(BaseHTTPMiddleware):
    """Middleware to enforce request size limit."""

    async def dispatch(self, request: Request, call_next):
        if request.method in ("POST", "PUT", "PATCH"):
            content_length = request.headers.get("content-length")
            if content_length and int(content_length) > settings.max_file_size_bytes:
                return JSONResponse(
                    status_code=413,
                    content={
                        "success": False,
                        "error": f"Payload too large: {content_length} bytes exceeds maximum of {settings.max_file_size_bytes} bytes",
                        "data": None,
                    },
                )
        return await call_next(request)


app.add_middleware(RequestSizeLimitMiddleware)

# Include routers
app.include_router(health.router, prefix="/api")
app.include_router(files.router, prefix="/api")

# Static file serving (SvelteKit SPA)
static_dir = Path(__file__).parent.parent.parent / "static"
if static_dir.exists():
    logger.info(f"Serving static files from {static_dir}")

    @app.get("/{full_path:path}", include_in_schema=False)
    async def serve_spa(full_path: str):
        """Serve static files and fall back to index.html for SPA routing."""
        file_path = static_dir / full_path
        if file_path.is_file():
            return FileResponse(file_path)
        # Fall back to index.html for SPA routing
        index_path = static_dir / "index.html"
        if index_path.is_file():
            return FileResponse(index_path)
        # Return 404 if neither file nor index.html exists
        return JSONResponse({"detail": "Not found"}, status_code=404)

    app.mount("/", StaticFiles(directory=static_dir, html=True), name="static")
else:
    logger.info("No static directory found, running API only")


@app.get("/", include_in_schema=False)
async def root():
    """Root endpoint."""
    return {"message": "AI File Processor API"}


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(
        app,
        host=settings.host,
        port=settings.port,
        log_level=settings.log_level.lower(),
    )
