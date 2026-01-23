"""Request model definitions."""

from pydantic import BaseModel, Field


class ProcessQuery(BaseModel):
    """Query parameters for file processing endpoint."""

    provider: str = Field(..., description="AI provider to use (openai, anthropic)")
    operation: str = Field(..., description="Operation to perform (summarize, extract, analyze, classify, custom)")
    custom_prompt: str | None = Field(None, description="Custom prompt for custom operations")
