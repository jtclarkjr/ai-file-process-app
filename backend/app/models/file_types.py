"""File type definitions and utilities."""

from enum import Enum


class SupportedFileType(str, Enum):
    """Enumeration of supported file types."""

    PDF = "pdf"
    DOCX = "docx"
    TEXT = "text"
    MARKDOWN = "markdown"
    CODE = "code"
    IMAGE_JPEG = "image_jpeg"
    IMAGE_PNG = "image_png"
    IMAGE_GIF = "image_gif"
    IMAGE_WEBP = "image_webp"

    @classmethod
    def from_mime(cls, mime_type: str) -> "SupportedFileType | None":
        """Get file type from MIME type."""
        mime_map = {
            "application/pdf": cls.PDF,
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document": cls.DOCX,
            "text/plain": cls.TEXT,
            "text/markdown": cls.MARKDOWN,
            "image/jpeg": cls.IMAGE_JPEG,
            "image/png": cls.IMAGE_PNG,
            "image/gif": cls.IMAGE_GIF,
            "image/webp": cls.IMAGE_WEBP,
            # Code files
            "text/x-python": cls.CODE,
            "application/x-python": cls.CODE,
            "text/javascript": cls.CODE,
            "application/javascript": cls.CODE,
            "text/x-typescript": cls.CODE,
            "application/typescript": cls.CODE,
            "text/x-rust": cls.CODE,
            "text/x-c": cls.CODE,
            "text/x-c++": cls.CODE,
            "text/x-java": cls.CODE,
            "text/x-go": cls.CODE,
            "application/json": cls.CODE,
            "text/yaml": cls.CODE,
            "application/x-yaml": cls.CODE,
            "text/html": cls.CODE,
            "text/css": cls.CODE,
            "text/xml": cls.CODE,
            "application/xml": cls.CODE,
        }
        return mime_map.get(mime_type)

    @classmethod
    def from_extension(cls, extension: str) -> "SupportedFileType | None":
        """Get file type from file extension."""
        ext = extension.lower().lstrip(".")
        ext_map = {
            # Documents
            "pdf": cls.PDF,
            "docx": cls.DOCX,
            "txt": cls.TEXT,
            "md": cls.MARKDOWN,
            "markdown": cls.MARKDOWN,
            # Code files
            "py": cls.CODE,
            "pyw": cls.CODE,
            "js": cls.CODE,
            "mjs": cls.CODE,
            "cjs": cls.CODE,
            "ts": cls.CODE,
            "tsx": cls.CODE,
            "mts": cls.CODE,
            "rs": cls.CODE,
            "c": cls.CODE,
            "h": cls.CODE,
            "cpp": cls.CODE,
            "hpp": cls.CODE,
            "cc": cls.CODE,
            "java": cls.CODE,
            "go": cls.CODE,
            "json": cls.CODE,
            "yaml": cls.CODE,
            "yml": cls.CODE,
            "html": cls.CODE,
            "htm": cls.CODE,
            "css": cls.CODE,
            "scss": cls.CODE,
            "sass": cls.CODE,
            "xml": cls.CODE,
            "svg": cls.CODE,
            "sh": cls.CODE,
            "bash": cls.CODE,
            "zsh": cls.CODE,
            "sql": cls.CODE,
            "rb": cls.CODE,
            "php": cls.CODE,
            "swift": cls.CODE,
            "kt": cls.CODE,
            "kts": cls.CODE,
            "scala": cls.CODE,
            "toml": cls.CODE,
            # Images
            "jpg": cls.IMAGE_JPEG,
            "jpeg": cls.IMAGE_JPEG,
            "png": cls.IMAGE_PNG,
            "gif": cls.IMAGE_GIF,
            "webp": cls.IMAGE_WEBP,
        }
        return ext_map.get(ext)

    def is_image(self) -> bool:
        """Check if this file type is an image."""
        return self in (
            self.IMAGE_JPEG,
            self.IMAGE_PNG,
            self.IMAGE_GIF,
            self.IMAGE_WEBP,
        )

    def mime_type(self) -> str:
        """Get the MIME type for this file type."""
        mime_map = {
            self.PDF: "application/pdf",
            self.DOCX: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            self.TEXT: "text/plain",
            self.MARKDOWN: "text/markdown",
            self.CODE: "text/plain",
            self.IMAGE_JPEG: "image/jpeg",
            self.IMAGE_PNG: "image/png",
            self.IMAGE_GIF: "image/gif",
            self.IMAGE_WEBP: "image/webp",
        }
        return mime_map[self]

    @staticmethod
    def all_supported() -> list[dict]:
        """Get list of all supported file types with metadata."""
        return [
            {
                "file_type": SupportedFileType.PDF,
                "extensions": ["pdf"],
                "description": "PDF documents",
            },
            {
                "file_type": SupportedFileType.DOCX,
                "extensions": ["docx"],
                "description": "Microsoft Word documents",
            },
            {
                "file_type": SupportedFileType.TEXT,
                "extensions": ["txt"],
                "description": "Plain text files",
            },
            {
                "file_type": SupportedFileType.MARKDOWN,
                "extensions": ["md", "markdown"],
                "description": "Markdown files",
            },
            {
                "file_type": SupportedFileType.CODE,
                "extensions": [
                    "py", "js", "ts", "tsx", "rs", "c", "cpp", "java", "go", "json",
                    "yaml", "yml", "html", "css", "xml", "sh", "sql", "rb", "php",
                    "swift", "kt", "scala", "toml"
                ],
                "description": "Source code files",
            },
            {
                "file_type": SupportedFileType.IMAGE_JPEG,
                "extensions": ["jpg", "jpeg"],
                "description": "JPEG images",
            },
            {
                "file_type": SupportedFileType.IMAGE_PNG,
                "extensions": ["png"],
                "description": "PNG images",
            },
            {
                "file_type": SupportedFileType.IMAGE_GIF,
                "extensions": ["gif"],
                "description": "GIF images",
            },
            {
                "file_type": SupportedFileType.IMAGE_WEBP,
                "extensions": ["webp"],
                "description": "WebP images",
            },
        ]
