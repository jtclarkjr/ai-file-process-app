"""OpenAI provider implementation."""

import base64
import logging
from typing import Any

from openai import AsyncOpenAI, APIError, APITimeoutError

from app.exceptions import AiProviderError, TimeoutError as AppTimeoutError
from app.models.operations import Operation

from .ai_provider import AiProvider, AiRequest, AiResponse, FileContent

logger = logging.getLogger(__name__)


class OpenAiProvider(AiProvider):
    """OpenAI implementation of AiProvider."""

    def __init__(self, api_key: str, model: str, timeout_secs: int):
        """Initialize OpenAI provider.

        Args:
            api_key: OpenAI API key
            model: Model name (e.g., gpt-4-turbo)
            timeout_secs: Request timeout in seconds
        """
        self.api_key = api_key
        self.model = model
        self.timeout = timeout_secs
        self.client = AsyncOpenAI(api_key=api_key, timeout=timeout_secs)

    @property
    def name(self) -> str:
        """Get provider name."""
        return "openai"

    @property
    def supports_vision(self) -> bool:
        """Check if model supports image processing."""
        return (
            "vision" in self.model
            or "gpt-4-turbo" in self.model
            or "gpt-4o" in self.model
        )

    async def process(self, request: AiRequest) -> AiResponse:
        """Process request with OpenAI API.

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
                full_prompt = f"{prompt}\n\n"
                if request.file_name:
                    full_prompt += f"File: {request.file_name}\n\n"
                full_prompt += request.content.text

                messages = [{"role": "user", "content": full_prompt}]
            else:
                # Image content
                if not self.supports_vision:
                    raise AiProviderError(
                        f"Model {self.model} does not support vision/image processing"
                    )

                image_data, media_type = request.content.image
                base64_data = base64.standard_b64encode(image_data).decode("utf-8")
                data_url = f"data:{media_type};base64,{base64_data}"

                text_part = f"{prompt}"
                if request.file_name:
                    text_part += f"\n\nFile: {request.file_name}"

                messages = [
                    {
                        "role": "user",
                        "content": [
                            {"type": "text", "text": text_part},
                            {"type": "image_url", "image_url": {"url": data_url}},
                        ],
                    }
                ]

            # Prepare max_tokens based on model
            kwargs: dict[str, Any] = {
                "model": self.model,
                "messages": messages,
            }

            if self.model.startswith("gpt-5"):
                kwargs["max_completion_tokens"] = 4096
            else:
                kwargs["max_tokens"] = 4096

            # Call OpenAI API
            response = await self.client.chat.completions.create(**kwargs)

            # Extract result
            result = response.choices[0].message.content or ""
            usage = None
            if response.usage:
                usage = {
                    "input_tokens": response.usage.prompt_tokens,
                    "output_tokens": response.usage.completion_tokens,
                }

            return AiResponse(result=result, model=response.model, usage=usage)

        except APITimeoutError as e:
            logger.error(f"OpenAI API timeout: {e}")
            raise AppTimeoutError() from e
        except APIError as e:
            logger.error(f"OpenAI API error: {e}")
            raise AiProviderError(str(e)) from e
        except Exception as e:
            logger.error(f"Unexpected error in OpenAI provider: {e}")
            raise AiProviderError(str(e)) from e
