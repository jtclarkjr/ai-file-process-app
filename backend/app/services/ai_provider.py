"""Abstract base class for AI providers."""

from abc import ABC, abstractmethod


class FileContent:
    """File content union type."""

    def __init__(self, content: str | tuple[bytes, str]):
        """Initialize with either text or image data."""
        if isinstance(content, str):
            self.text = content
            self.image = None
        else:
            self.text = None
            self.image = content  # (bytes, media_type)


class AiRequest:
    """Request to an AI provider."""

    def __init__(
        self,
        content: FileContent,
        operation: str,
        custom_prompt: str | None = None,
        file_name: str | None = None,
    ):
        self.content = content
        self.operation = operation
        self.custom_prompt = custom_prompt
        self.file_name = file_name


class AiResponse:
    """Response from an AI provider."""

    def __init__(self, result: str, model: str, usage: dict | None = None):
        self.result = result
        self.model = model
        self.usage = usage


class AiProvider(ABC):
    """Abstract base class for AI providers."""

    @property
    @abstractmethod
    def name(self) -> str:
        """Get provider name."""
        pass

    @property
    @abstractmethod
    def supports_vision(self) -> bool:
        """Check if provider supports image processing."""
        pass

    @abstractmethod
    async def process(self, request: AiRequest) -> AiResponse:
        """Process a request with the AI provider."""
        pass
