"""Operation definitions and utilities."""

from enum import Enum


class Operation(str, Enum):
    """Enumeration of supported operations."""

    SUMMARIZE = "summarize"
    EXTRACT = "extract"
    ANALYZE = "analyze"
    CLASSIFY = "classify"
    CUSTOM = "custom"

    def to_prompt(self, custom_prompt: str | None = None) -> str:
        """Convert operation to instruction prompt."""
        prompts = {
            Operation.SUMMARIZE: (
                "Provide a concise summary of the following content. "
                "Focus on the main points and key takeaways."
            ),
            Operation.EXTRACT: (
                "Extract all important information from the following content. "
                "Include key facts, figures, names, dates, and any structured data. "
                "Format the output as a structured list."
            ),
            Operation.ANALYZE: (
                "Analyze the following content in depth. "
                "Identify themes, patterns, sentiment, and provide insights. "
                "Include both objective observations and interpretive analysis."
            ),
            Operation.CLASSIFY: (
                "Classify the following content. Determine:\n"
                "1. Document type (e.g., report, letter, article, code, etc.)\n"
                "2. Primary topic/subject\n"
                "3. Target audience\n"
                "4. Tone (formal, informal, technical, etc.)\n"
                "5. Key categories or tags that apply"
            ),
            Operation.CUSTOM: custom_prompt or "Process this content.",
        }
        return prompts[self]

    @classmethod
    def from_string(cls, value: str) -> "Operation":
        """Parse operation from string."""
        try:
            return cls(value.lower())
        except ValueError:
            valid_ops = ", ".join([op.value for op in cls])
            raise ValueError(
                f"Invalid operation: {value}. Valid options: {valid_ops}"
            )
