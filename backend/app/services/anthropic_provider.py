"""Anthropic provider implementation."""

import base64
import logging
from typing import Any

from anthropic import AsyncAnthropic, APIError, APITimeoutError

from app.exceptions import AiProviderError, TimeoutError as AppTimeoutError
from app.models.operations import Operation

from .ai_provider import AiProvider, AiRequest, AiResponse, FileContent

logger = logging.getLogger(__name__)


class AnthropicProvider(AiProvider):
    """Anthropic implementation of AiProvider."""

    def __init__(self, api_key: str, model: str, timeout_secs: int):
        """Initialize Anthropic provider.

        Args:
            api_key: Anthropic API key
            model: Model name (e.g., claude-3-sonnet-20240229)
            timeout_secs: Request timeout in seconds
        """
        self.api_key = api_key
        self.model = model
        self.timeout = timeout_secs
        self.client = AsyncAnthropic(api_key=api_key, timeout=timeout_secs)

    @property
    def name(self) -> str:
        """Get provider name."""
        return "anthropic"

    @property
    def supports_vision(self) -> bool:
        """Check if model supports image processing."""
        return "claude-3" in self.model

    async def process(self, request: AiRequest) -> AiResponse:
        """Process request with Anthropic API.

        Args:
            request: AI request containing content and operation

        Returns:
            AI response with result and metadata

        Raises:
            AiProviderError: If API call fails
            AppTimeoutError: If request times out
        """
        try:
            # Get operation prompt
            operation = Operation.from_string(request.operation)
            prompt = operation.to_prompt(request.custom_prompt)

            # Build message content
            if request.content.text is not None:
                # Text content
                full_text = f"{prompt}\n\n"
                if request.file_name:
                    full_text += f"File: {request.file_name}\n\n"
                full_text += request.content.text

                content: list[dict[str, Any]] = [{"type": "text", "text": full_text}]
            else:
                # Image content
                if not self.supports_vision:
                    raise AiProviderError(
                        f"Model {self.model} does not support vision/image processing"
                    )

                image_data, media_type = request.content.image
                base64_data = base64.standard_b64encode(image_data).decode("utf-8")

                text_part = f"{prompt}"
                if request.file_name:
                    text_part += f"\n\nFile: {request.file_name}"

                content = [
                    {"type": "text", "text": text_part},
                    {
                        "type": "image",
                        "source": {
                            "type": "base64",
                            "media_type": media_type,
                            "data": base64_data,
                        },
                    },
                ]

            # Call Anthropic API
            response = await self.client.messages.create(
                model=self.model,
                max_tokens=4096,
                messages=[{"role": "user", "content": content}],
            )

            # Extract result
            result = ""
            for block in response.content:
                if block.type == "text":
                    result += block.text

            if not result:
                raise AiProviderError("No text response from Anthropic")

            usage = {
                "input_tokens": response.usage.input_tokens,
                "output_tokens": response.usage.output_tokens,
            }

            return AiResponse(result=result, model=response.model, usage=usage)

        except APITimeoutError as e:
            logger.error(f"Anthropic API timeout: {e}")
            raise AppTimeoutError() from e
        except APIError as e:
            logger.error(f"Anthropic API error: {e}")
            raise AiProviderError(str(e)) from e
        except Exception as e:
            logger.error(f"Unexpected error in Anthropic provider: {e}")
            raise AiProviderError(str(e)) from e
