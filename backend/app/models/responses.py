"""Response model definitions."""

from pydantic import BaseModel

from .file_types import SupportedFileType


class TokenUsage(BaseModel):
    """Token usage information from AI provider."""

    input_tokens: int
    output_tokens: int


class ProcessResult(BaseModel):
    """Result of file processing."""

    result: str
    model: str
    file_type: SupportedFileType
    original_size: int
    processing_time_ms: int
    usage: TokenUsage | None = None


class ProcessResponse(BaseModel):
    """Response from file processing endpoint."""

    success: bool
    data: ProcessResult | None = None
    error: str | None = None


class SupportedTypeInfo(BaseModel):
    """Information about a supported file type."""

    file_type: SupportedFileType
    extensions: list[str]
    description: str


class SupportedTypesResponse(BaseModel):
    """Response containing list of supported file types."""

    success: bool
    data: list[SupportedTypeInfo]


class ProviderInfo(BaseModel):
    """Information about an AI provider."""

    id: str
    name: str
    available: bool
    supports_vision: bool


class ProvidersResponse(BaseModel):
    """Response containing list of available AI providers."""

    success: bool
    data: list[ProviderInfo]


class HealthResponse(BaseModel):
    """Health check response."""

    status: str
