"""Tests for AI providers."""

import pytest

from app.models.operations import Operation
from app.services.ai_provider import AiRequest, FileContent


class TestOperation:
    """Tests for Operation enum."""

    def test_summarize_prompt(self):
        """Test summarize operation prompt."""
        prompt = Operation.SUMMARIZE.to_prompt()
        assert "summary" in prompt.lower()
        assert "main points" in prompt.lower()

    def test_extract_prompt(self):
        """Test extract operation prompt."""
        prompt = Operation.EXTRACT.to_prompt()
        assert "extract" in prompt.lower()

    def test_analyze_prompt(self):
        """Test analyze operation prompt."""
        prompt = Operation.ANALYZE.to_prompt()
        assert "analyze" in prompt.lower()

    def test_classify_prompt(self):
        """Test classify operation prompt."""
        prompt = Operation.CLASSIFY.to_prompt()
        assert "classify" in prompt.lower()

    def test_custom_prompt(self):
        """Test custom operation with custom prompt."""
        custom = "Do something specific"
        prompt = Operation.CUSTOM.to_prompt(custom)
        assert prompt == custom

    def test_from_string_valid(self):
        """Test parsing valid operation strings."""
        assert Operation.from_string("summarize") == Operation.SUMMARIZE
        assert Operation.from_string("EXTRACT") == Operation.EXTRACT
        assert Operation.from_string("Analyze") == Operation.ANALYZE

    def test_from_string_invalid(self):
        """Test parsing invalid operation string raises error."""
        with pytest.raises(ValueError) as exc_info:
            Operation.from_string("invalid")
        assert "Invalid operation" in str(exc_info.value)


class TestFileContent:
    """Tests for FileContent class."""

    def test_text_content(self):
        """Test creating text content."""
        content = FileContent("Hello, world!")
        assert content.text == "Hello, world!"
        assert content.image is None

    def test_image_content(self):
        """Test creating image content."""
        image_data = b"\x89PNG..."
        content = FileContent((image_data, "image/png"))
        assert content.text is None
        assert content.image == (image_data, "image/png")


class TestAiRequest:
    """Tests for AiRequest class."""

    def test_request_with_text(self):
        """Test creating request with text content."""
        content = FileContent("Test content")
        request = AiRequest(
            content=content,
            operation="summarize",
            file_name="test.txt",
        )
        assert request.content.text == "Test content"
        assert request.operation == "summarize"
        assert request.file_name == "test.txt"
        assert request.custom_prompt is None

    def test_request_with_custom_prompt(self):
        """Test creating request with custom prompt."""
        content = FileContent("Test content")
        request = AiRequest(
            content=content,
            operation="custom",
            custom_prompt="Do something",
        )
        assert request.custom_prompt == "Do something"
