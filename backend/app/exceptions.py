"""Custom exception classes for the application."""

from fastapi import HTTPException, status


class AppException(HTTPException):
    """Base application exception."""

    def __init__(self, message: str, status_code: int = status.HTTP_500_INTERNAL_SERVER_ERROR):
        super().__init__(status_code=status_code, detail={"success": False, "error": message, "data": None})


class FileTooLargeError(AppException):
    """Raised when file size exceeds the maximum limit."""

    def __init__(self, size: int, max_size: int):
        message = f"File too large: {size} bytes exceeds maximum of {max_size} bytes"
        super().__init__(message, status_code=status.HTTP_413_REQUEST_ENTITY_TOO_LARGE)


class UnsupportedFileTypeError(AppException):
    """Raised when file type is not supported."""

    def __init__(self, file_type: str):
        message = f"Unsupported file type: {file_type}"
        super().__init__(message, status_code=status.HTTP_415_UNSUPPORTED_MEDIA_TYPE)


class MimeTypeMismatchError(AppException):
    """Raised when declared MIME type doesn't match detected MIME type."""

    def __init__(self, declared: str, detected: str):
        message = f"MIME type mismatch: declared {declared}, detected {detected}"
        super().__init__(message, status_code=status.HTTP_415_UNSUPPORTED_MEDIA_TYPE)


class DecompressionBombError(AppException):
    """Raised when a potential decompression bomb is detected."""

    def __init__(self, ratio: int, max_ratio: int):
        message = f"Decompression bomb detected: ratio {ratio}:1 exceeds maximum of {max_ratio}:1"
        super().__init__(message, status_code=status.HTTP_400_BAD_REQUEST)


class FileExtractionError(AppException):
    """Raised when file extraction fails."""

    def __init__(self, message: str):
        super().__init__(f"File extraction failed: {message}", status_code=status.HTTP_422_UNPROCESSABLE_ENTITY)


class AiProviderError(AppException):
    """Raised when AI provider returns an error."""

    def __init__(self, message: str):
        super().__init__(f"AI provider error: {message}", status_code=status.HTTP_502_BAD_GATEWAY)


class ProviderNotConfiguredError(AppException):
    """Raised when requested AI provider is not configured."""

    def __init__(self, provider: str):
        message = f"AI provider not configured: {provider}"
        super().__init__(message, status_code=status.HTTP_503_SERVICE_UNAVAILABLE)


class InvalidRequestError(AppException):
    """Raised when request parameters are invalid."""

    def __init__(self, message: str):
        super().__init__(f"Invalid request: {message}", status_code=status.HTTP_400_BAD_REQUEST)


class TimeoutError(AppException):
    """Raised when a request times out."""

    def __init__(self):
        super().__init__("Request timeout", status_code=status.HTTP_504_GATEWAY_TIMEOUT)
