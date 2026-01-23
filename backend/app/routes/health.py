"""Health check endpoint."""

from fastapi import APIRouter

from app.models.responses import HealthResponse

router = APIRouter(tags=["health"])


@router.get("/health")
async def health() -> HealthResponse:
    """Health check endpoint."""
    return HealthResponse(status="OK")
