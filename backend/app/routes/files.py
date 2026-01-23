"""File processing endpoints."""

import logging
import time
from typing import Any

from fastapi import APIRouter, File, Query, UploadFile

from app.exceptions import (
    AiProviderError,
    InvalidRequestError,
    ProviderNotConfiguredError,
)
from app.models import (
    ProcessResponse,
    ProcessResult,
    ProviderInfo,
    ProvidersResponse,
    SupportedFileType,
    SupportedTypeInfo,
    SupportedTypesResponse,
    TokenUsage,
)
from app.services import (
    AiRequest,
    AnthropicProvider,
    FileExtractor,
    OpenAiProvider,
)

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/files", tags=["files"])

# Global providers - initialized in main.py
_openai_provider: OpenAiProvider | None = None
_anthropic_provider: AnthropicProvider | None = None
_file_extractor: FileExtractor | None = None


def set_providers(
    openai: OpenAiProvider | None = None,
    anthropic: AnthropicProvider | None = None,
    extractor: FileExtractor | None = None,
) -> None:
    """Set the global providers."""
    global _openai_provider, _anthropic_provider, _file_extractor
    _openai_provider = openai
    _anthropic_provider = anthropic
    _file_extractor = extractor


@router.post("/process", response_model=ProcessResponse)
async def process_file(
    provider: str = Query(..., description="AI provider to use"),
    operation: str = Query(..., description="Operation to perform"),
    custom_prompt: str | None = Query(None, description="Custom prompt"),
    file: UploadFile = File(..., description="File to process"),
) -> ProcessResponse:
    """Process a file with the specified AI provider.

    Args:
        provider: AI provider (openai, anthropic)
        operation: Operation (summarize, extract, analyze, classify, custom)
        custom_prompt: Optional custom prompt
        file: File to process

    Returns:
        ProcessResponse with result or error
    """
    start_time = time.time()

    try:
        # Validate providers are initialized
        if _file_extractor is None or (
            provider.lower() == "openai" and _openai_provider is None
        ) and (provider.lower() == "anthropic" and _anthropic_provider is None):
            return ProcessResponse(
                success=False,
                error="Providers not initialized",
                data=None,
            )

        # Read file data
        file_data = await file.read()
        file_name = file.filename

        # Log metadata only (no content)
        logger.info(
            f"Processing file: {file_name} ({len(file_data)} bytes) "
            f"with provider={provider}, operation={operation}"
        )

        # Validate and detect file type
        file_type = _file_extractor.validate_and_detect_type(
            file_data, file.content_type, file_name
        )

        # Get the appropriate provider
        if provider.lower() == "openai":
            if _openai_provider is None:
                raise ProviderNotConfiguredError("OpenAI")
            ai_provider = _openai_provider
        elif provider.lower() == "anthropic":
            if _anthropic_provider is None:
                raise ProviderNotConfiguredError("Anthropic")
            ai_provider = _anthropic_provider
        else:
            raise InvalidRequestError(
                f"Unknown provider: {provider}. Valid options: openai, anthropic"
            )

        # Check if provider supports images when needed
        if file_type.is_image() and not ai_provider.supports_vision:
            raise AiProviderError(
                f"Provider {ai_provider.name} does not support vision/image processing"
            )

        # Extract content from file
        extracted_content, original_size = _file_extractor.extract(file_data, file_type)

        # Create AI request
        ai_request = AiRequest(
            content=extracted_content,
            operation=operation,
            custom_prompt=custom_prompt,
            file_name=file_name,
        )

        # Process with AI provider
        ai_response = await ai_provider.process(ai_request)

        # Calculate processing time
        processing_time_ms = int((time.time() - start_time) * 1000)

        # Build response
        usage = None
        if ai_response.usage:
            usage = TokenUsage(
                input_tokens=ai_response.usage["input_tokens"],
                output_tokens=ai_response.usage["output_tokens"],
            )

        result = ProcessResult(
            result=ai_response.result,
            model=ai_response.model,
            file_type=file_type,
            original_size=original_size,
            processing_time_ms=processing_time_ms,
            usage=usage,
        )

        return ProcessResponse(success=True, data=result, error=None)

    except Exception as e:
        logger.error(f"Error processing file: {e}", exc_info=True)
        # The exception should be caught by FastAPI's exception handler
        raise


@router.get("/supported-types", response_model=SupportedTypesResponse)
async def get_supported_types() -> SupportedTypesResponse:
    """Get list of supported file types."""
    supported = SupportedFileType.all_supported()

    data = [
        SupportedTypeInfo(
            file_type=item["file_type"],
            extensions=item["extensions"],
            description=item["description"],
        )
        for item in supported
    ]

    return SupportedTypesResponse(success=True, data=data)


@router.get("/providers", response_model=ProvidersResponse)
async def get_providers() -> ProvidersResponse:
    """Get list of available AI providers."""
    providers = []

    # OpenAI provider
    providers.append(
        ProviderInfo(
            id="openai",
            name="OpenAI",
            available=_openai_provider is not None,
            supports_vision=_openai_provider.supports_vision if _openai_provider else False,
        )
    )

    # Anthropic provider
    providers.append(
        ProviderInfo(
            id="anthropic",
            name="Anthropic",
            available=_anthropic_provider is not None,
            supports_vision=_anthropic_provider.supports_vision if _anthropic_provider else False,
        )
    )

    return ProvidersResponse(success=True, data=providers)
