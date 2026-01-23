"""File extraction and validation service."""

import io
import logging
import mimetypes

import magic
from docx import Document as DocxDocument
from docx.document import Document
from pypdf import PdfReader

from app.config import settings
from app.exceptions import (
    DecompressionBombError,
    FileExtractionError,
    FileTooLargeError,
    MimeTypeMismatchError,
    UnsupportedFileTypeError,
)
from app.models.file_types import SupportedFileType

from .ai_provider import FileContent

logger = logging.getLogger(__name__)


class FileExtractor:
    """Service for extracting content from various file types."""

    def __init__(self, config=None):
        """Initialize file extractor with configuration."""
        self.config = config or settings

    def validate_and_detect_type(
        self,
        data: bytes,
        declared_mime: str | None = None,
        file_name: str | None = None,
    ) -> SupportedFileType:
        """Validate and detect file type from data.

        Args:
            data: Raw file data
            declared_mime: Declared MIME type from upload
            file_name: Original file name

        Returns:
            Detected SupportedFileType

        Raises:
            FileTooLargeError: If file exceeds max size
            UnsupportedFileTypeError: If file type not supported
            MimeTypeMismatchError: If declared MIME doesn't match detected
        """
        # Check file size
        if len(data) > self.config.max_file_size_bytes:
            raise FileTooLargeError(len(data), self.config.max_file_size_bytes)

        # Detect MIME type from magic bytes
        try:
            mime = magic.Magic(mime=True)
            detected_mime = mime.from_buffer(data)
        except Exception as e:
            logger.warning(f"Magic detection failed: {e}, falling back to extension")
            detected_mime = None

        # Try to determine file type from detected MIME
        file_type = None
        if detected_mime:
            file_type = SupportedFileType.from_mime(detected_mime)

        # If not detected from magic bytes, try extension
        if file_type is None and file_name:
            ext = file_name.rsplit(".", 1)[-1] if "." in file_name else None
            if ext:
                file_type = SupportedFileType.from_extension(ext)

        # If still not detected, try declared MIME
        if file_type is None and declared_mime:
            file_type = SupportedFileType.from_mime(declared_mime)

        if file_type is None:
            unknown_type = detected_mime or declared_mime or "unknown"
            raise UnsupportedFileTypeError(unknown_type)

        # For binary files, validate MIME type matches if declared
        if declared_mime and detected_mime:
            # Only validate for file types where magic bytes are reliable
            if file_type.is_image() or file_type == SupportedFileType.PDF:
                declared_type = SupportedFileType.from_mime(declared_mime)
                detected_type = SupportedFileType.from_mime(detected_mime)

                if declared_type != detected_type:
                    raise MimeTypeMismatchError(declared_mime, detected_mime)

        return file_type

    def extract(self, data: bytes, file_type: SupportedFileType) -> tuple[FileContent, int]:
        """Extract content from file.

        Args:
            data: Raw file data
            file_type: Detected file type

        Returns:
            Tuple of (FileContent, original_size)

        Raises:
            FileExtractionError: If extraction fails
            DecompressionBombError: If potential decompression bomb detected
            FileTooLargeError: If decompressed size exceeds limit
        """
        original_size = len(data)

        try:
            if file_type == SupportedFileType.PDF:
                content = self._extract_pdf(data)
            elif file_type == SupportedFileType.DOCX:
                content = self._extract_docx(data)
            elif file_type in (
                SupportedFileType.TEXT,
                SupportedFileType.MARKDOWN,
                SupportedFileType.CODE,
            ):
                content = self._extract_text(data)
            elif file_type.is_image():
                content = FileContent((data, file_type.mime_type()))
            else:
                raise FileExtractionError(f"Unsupported file type: {file_type}")
        except FileExtractionError:
            raise
        except Exception as e:
            raise FileExtractionError(str(e)) from e

        # Check for decompression bomb (text content much larger than original)
        if content.text is not None:
            text_len = len(content.text.encode("utf-8"))
            ratio = text_len // max(original_size, 1)

            if ratio > self.config.max_decompression_ratio:
                raise DecompressionBombError(ratio, self.config.max_decompression_ratio)

            if text_len > self.config.max_decompressed_size_bytes:
                raise FileTooLargeError(text_len, self.config.max_decompressed_size_bytes)

        return content, original_size

    def _extract_pdf(self, data: bytes) -> FileContent:
        """Extract text from PDF."""
        try:
            pdf_file = io.BytesIO(data)
            reader = PdfReader(pdf_file)

            text = ""
            for page in reader.pages:
                text += page.extract_text() or ""

            if not text.strip():
                raise FileExtractionError(
                    "PDF contains no extractable text (may be image-based)"
                )

            return FileContent(text)
        except FileExtractionError:
            raise
        except Exception as e:
            raise FileExtractionError(f"PDF extraction failed: {str(e)}") from e

    def _extract_docx(self, data: bytes) -> FileContent:
        """Extract text from DOCX."""
        try:
            docx_file = io.BytesIO(data)
            doc: Document = DocxDocument(docx_file)

            text = ""
            for paragraph in doc.paragraphs:
                text += paragraph.text + "\n"

            if not text.strip():
                raise FileExtractionError("DOCX contains no extractable text")

            return FileContent(text)
        except FileExtractionError:
            raise
        except Exception as e:
            raise FileExtractionError(f"DOCX parsing failed: {str(e)}") from e

    def _extract_text(self, data: bytes) -> FileContent:
        """Extract text from plain text file."""
        try:
            text = data.decode("utf-8")
            return FileContent(text)
        except UnicodeDecodeError as e:
            raise FileExtractionError(f"Invalid UTF-8 in text file: {str(e)}") from e
