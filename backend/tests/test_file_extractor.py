"""Tests for file extractor service."""

import pytest

from app.models.file_types import SupportedFileType
from app.services.file_extractor import FileExtractor


@pytest.fixture
def extractor():
    """Create a file extractor instance."""
    return FileExtractor()


class TestFileTypeDetection:
    """Tests for file type detection."""

    def test_detect_text_by_extension(self, extractor):
        """Test detecting text file by extension."""
        data = b"Hello, world!"
        file_type = extractor.validate_and_detect_type(data, None, "test.txt")
        assert file_type == SupportedFileType.TEXT

    def test_detect_markdown_by_extension(self, extractor):
        """Test detecting markdown file by extension."""
        data = b"# Hello\n\nWorld"
        file_type = extractor.validate_and_detect_type(data, None, "test.md")
        assert file_type == SupportedFileType.MARKDOWN

    def test_detect_python_code_by_extension(self, extractor):
        """Test detecting Python code by extension."""
        data = b"print('hello')"
        file_type = extractor.validate_and_detect_type(data, None, "test.py")
        assert file_type == SupportedFileType.CODE

    def test_detect_json_by_extension(self, extractor):
        """Test detecting JSON by extension."""
        data = b'{"key": "value"}'
        file_type = extractor.validate_and_detect_type(data, None, "test.json")
        assert file_type == SupportedFileType.CODE


class TestTextExtraction:
    """Tests for text extraction."""

    def test_extract_plain_text(self, extractor):
        """Test extracting plain text."""
        data = b"Hello, world!"
        content, size = extractor.extract(data, SupportedFileType.TEXT)
        assert content.text == "Hello, world!"
        assert size == len(data)

    def test_extract_utf8_text(self, extractor):
        """Test extracting UTF-8 text with special characters."""
        text = "Hello, 世界! 🌍"
        data = text.encode("utf-8")
        content, size = extractor.extract(data, SupportedFileType.TEXT)
        assert content.text == text


class TestImageHandling:
    """Tests for image handling."""

    def test_image_returns_binary_content(self, extractor):
        """Test that images return binary content."""
        # Minimal valid PNG header
        png_header = bytes([
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A
        ])
        content, size = extractor.extract(png_header, SupportedFileType.IMAGE_PNG)
        assert content.image is not None
        assert content.image[0] == png_header
        assert content.image[1] == "image/png"
