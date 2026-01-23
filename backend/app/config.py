"""Configuration module for environment variables."""

from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    """Application configuration from environment variables."""

    # AI Provider settings
    openai_api_key: str | None = None
    openai_model: str = "gpt-4-turbo"
    anthropic_api_key: str | None = None
    anthropic_model: str = "claude-3-sonnet-20240229"

    # Server settings
    host: str = "0.0.0.0"
    port: int = 8080
    log_level: str = "INFO"

    # File processing limits
    max_file_size_mb: int = 10
    request_timeout_secs: int = 120
    ai_timeout_secs: int = 60

    # Decompression bomb protection
    max_decompression_ratio: int = 100  # 100:1 max ratio
    max_decompressed_size_mb: int = 50  # 50MB max

    class Config:
        env_file = ".env"
        env_file_encoding = "utf-8"
        case_sensitive = False

    @property
    def max_file_size_bytes(self) -> int:
        """Get max file size in bytes."""
        return self.max_file_size_mb * 1024 * 1024

    @property
    def max_decompressed_size_bytes(self) -> int:
        """Get max decompressed size in bytes."""
        return self.max_decompressed_size_mb * 1024 * 1024


# Global settings instance
settings = Settings()
