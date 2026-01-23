use bytes::Bytes;
use serde::{Deserialize, Serialize};

use super::FileContent;
use crate::config::Config;
use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SupportedFileType {
    Pdf,
    Docx,
    Text,
    Markdown,
    Code,
    ImageJpeg,
    ImagePng,
    ImageGif,
    ImageWebp,
}

impl SupportedFileType {
    pub fn from_mime(mime: &str) -> Option<Self> {
        match mime {
            "application/pdf" => Some(Self::Pdf),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
                Some(Self::Docx)
            }
            "text/plain" => Some(Self::Text),
            "text/markdown" => Some(Self::Markdown),
            "image/jpeg" => Some(Self::ImageJpeg),
            "image/png" => Some(Self::ImagePng),
            "image/gif" => Some(Self::ImageGif),
            "image/webp" => Some(Self::ImageWebp),
            // Common code file types
            "text/x-python" | "application/x-python" => Some(Self::Code),
            "text/javascript" | "application/javascript" => Some(Self::Code),
            "text/x-typescript" | "application/typescript" => Some(Self::Code),
            "text/x-rust" => Some(Self::Code),
            "text/x-c" | "text/x-c++" => Some(Self::Code),
            "text/x-java" => Some(Self::Code),
            "text/x-go" => Some(Self::Code),
            "application/json" => Some(Self::Code),
            "text/yaml" | "application/x-yaml" => Some(Self::Code),
            "text/html" | "text/css" => Some(Self::Code),
            "text/xml" | "application/xml" => Some(Self::Code),
            _ => None,
        }
    }

    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "pdf" => Some(Self::Pdf),
            "docx" => Some(Self::Docx),
            "txt" => Some(Self::Text),
            "md" | "markdown" => Some(Self::Markdown),
            "jpg" | "jpeg" => Some(Self::ImageJpeg),
            "png" => Some(Self::ImagePng),
            "gif" => Some(Self::ImageGif),
            "webp" => Some(Self::ImageWebp),
            // Code files
            "py" | "pyw" => Some(Self::Code),
            "js" | "mjs" | "cjs" => Some(Self::Code),
            "ts" | "tsx" | "mts" => Some(Self::Code),
            "rs" => Some(Self::Code),
            "c" | "h" | "cpp" | "hpp" | "cc" => Some(Self::Code),
            "java" => Some(Self::Code),
            "go" => Some(Self::Code),
            "json" => Some(Self::Code),
            "yaml" | "yml" => Some(Self::Code),
            "html" | "htm" | "css" | "scss" | "sass" => Some(Self::Code),
            "xml" | "svg" => Some(Self::Code),
            "sh" | "bash" | "zsh" => Some(Self::Code),
            "sql" => Some(Self::Code),
            "rb" => Some(Self::Code),
            "php" => Some(Self::Code),
            "swift" => Some(Self::Code),
            "kt" | "kts" => Some(Self::Code),
            "scala" => Some(Self::Code),
            "toml" => Some(Self::Code),
            _ => None,
        }
    }

    pub fn is_image(&self) -> bool {
        matches!(
            self,
            Self::ImageJpeg | Self::ImagePng | Self::ImageGif | Self::ImageWebp
        )
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Pdf => "application/pdf",
            Self::Docx => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            Self::Text => "text/plain",
            Self::Markdown => "text/markdown",
            Self::Code => "text/plain",
            Self::ImageJpeg => "image/jpeg",
            Self::ImagePng => "image/png",
            Self::ImageGif => "image/gif",
            Self::ImageWebp => "image/webp",
        }
    }

    pub fn all_supported() -> Vec<SupportedFileInfo> {
        vec![
            SupportedFileInfo {
                file_type: Self::Pdf,
                extensions: vec!["pdf"],
                description: "PDF documents",
            },
            SupportedFileInfo {
                file_type: Self::Docx,
                extensions: vec!["docx"],
                description: "Microsoft Word documents",
            },
            SupportedFileInfo {
                file_type: Self::Text,
                extensions: vec!["txt"],
                description: "Plain text files",
            },
            SupportedFileInfo {
                file_type: Self::Markdown,
                extensions: vec!["md", "markdown"],
                description: "Markdown files",
            },
            SupportedFileInfo {
                file_type: Self::Code,
                extensions: vec![
                    "py", "js", "ts", "tsx", "rs", "c", "cpp", "java", "go", "json", "yaml", "yml",
                    "html", "css", "xml", "sh", "sql", "rb", "php", "swift", "kt", "scala", "toml",
                ],
                description: "Source code files",
            },
            SupportedFileInfo {
                file_type: Self::ImageJpeg,
                extensions: vec!["jpg", "jpeg"],
                description: "JPEG images",
            },
            SupportedFileInfo {
                file_type: Self::ImagePng,
                extensions: vec!["png"],
                description: "PNG images",
            },
            SupportedFileInfo {
                file_type: Self::ImageGif,
                extensions: vec!["gif"],
                description: "GIF images",
            },
            SupportedFileInfo {
                file_type: Self::ImageWebp,
                extensions: vec!["webp"],
                description: "WebP images",
            },
        ]
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SupportedFileInfo {
    pub file_type: SupportedFileType,
    pub extensions: Vec<&'static str>,
    pub description: &'static str,
}

#[derive(Debug)]
pub struct ExtractedContent {
    pub content: FileContent,
    pub file_type: SupportedFileType,
    pub original_size: usize,
}

pub struct FileExtractor {
    config: Config,
}

impl FileExtractor {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn validate_and_detect_type(
        &self,
        data: &[u8],
        declared_mime: Option<&str>,
        file_name: Option<&str>,
    ) -> Result<SupportedFileType, AppError> {
        // Check file size
        if data.len() > self.config.max_file_size_bytes {
            return Err(AppError::FileTooLarge(
                data.len(),
                self.config.max_file_size_bytes,
            ));
        }

        // Detect MIME type from magic bytes
        let detected = infer::get(data);
        let detected_mime = detected.map(|t| t.mime_type());

        // Try to determine file type from detected MIME
        let file_type = if let Some(mime) = detected_mime {
            SupportedFileType::from_mime(mime)
        } else {
            None
        };

        // If not detected from magic bytes, try extension
        let file_type = file_type.or_else(|| {
            file_name
                .and_then(|name| name.rsplit('.').next())
                .and_then(SupportedFileType::from_extension)
        });

        // If still not detected, try declared MIME
        let file_type = file_type.or_else(|| declared_mime.and_then(SupportedFileType::from_mime));

        let file_type = file_type.ok_or_else(|| {
            AppError::UnsupportedFileType(
                detected_mime
                    .or(declared_mime)
                    .unwrap_or("unknown")
                    .to_string(),
            )
        })?;

        // For non-text files, validate MIME type matches if declared
        if let (Some(declared), Some(detected)) = (declared_mime, detected_mime) {
            // Only validate for binary files where magic bytes are reliable
            if file_type.is_image() || matches!(file_type, SupportedFileType::Pdf) {
                let declared_type = SupportedFileType::from_mime(declared);
                let detected_type = SupportedFileType::from_mime(detected);

                if declared_type != detected_type {
                    return Err(AppError::MimeTypeMismatch {
                        declared: declared.to_string(),
                        detected: detected.to_string(),
                    });
                }
            }
        }

        Ok(file_type)
    }

    pub fn extract(
        &self,
        data: Bytes,
        file_type: &SupportedFileType,
    ) -> Result<ExtractedContent, AppError> {
        let original_size = data.len();

        let content = match file_type {
            SupportedFileType::Pdf => self.extract_pdf(&data)?,
            SupportedFileType::Docx => self.extract_docx(&data)?,
            SupportedFileType::Text | SupportedFileType::Markdown | SupportedFileType::Code => {
                self.extract_text(&data)?
            }
            SupportedFileType::ImageJpeg
            | SupportedFileType::ImagePng
            | SupportedFileType::ImageGif
            | SupportedFileType::ImageWebp => FileContent::Image {
                data: data.to_vec(),
                media_type: file_type.mime_type().to_string(),
            },
        };

        // Check for decompression bomb (text content much larger than original)
        if let FileContent::Text(ref text) = content {
            let ratio = text.len().checked_div(original_size.max(1)).unwrap_or(0);
            if ratio > self.config.max_decompression_ratio {
                return Err(AppError::DecompressionBomb(
                    ratio,
                    self.config.max_decompression_ratio,
                ));
            }
            if text.len() > self.config.max_decompressed_size_bytes {
                return Err(AppError::FileTooLarge(
                    text.len(),
                    self.config.max_decompressed_size_bytes,
                ));
            }
        }

        Ok(ExtractedContent {
            content,
            file_type: file_type.clone(),
            original_size,
        })
    }

    fn extract_pdf(&self, data: &[u8]) -> Result<FileContent, AppError> {
        let text = pdf_extract::extract_text_from_mem(data)
            .map_err(|e| AppError::ExtractionFailed(format!("PDF extraction failed: {}", e)))?;

        if text.trim().is_empty() {
            return Err(AppError::ExtractionFailed(
                "PDF contains no extractable text (may be image-based)".to_string(),
            ));
        }

        Ok(FileContent::Text(text))
    }

    fn extract_docx(&self, data: &[u8]) -> Result<FileContent, AppError> {
        let docx = docx_rs::read_docx(data)
            .map_err(|e| AppError::ExtractionFailed(format!("DOCX parsing failed: {}", e)))?;

        // Extract text from document
        let mut text = String::new();
        for child in docx.document.children {
            if let docx_rs::DocumentChild::Paragraph(para) = child {
                for child in para.children {
                    if let docx_rs::ParagraphChild::Run(run) = child {
                        for child in run.children {
                            if let docx_rs::RunChild::Text(t) = child {
                                text.push_str(&t.text);
                            }
                        }
                    }
                }
                text.push('\n');
            }
        }

        if text.trim().is_empty() {
            return Err(AppError::ExtractionFailed(
                "DOCX contains no extractable text".to_string(),
            ));
        }

        Ok(FileContent::Text(text))
    }

    fn extract_text(&self, data: &[u8]) -> Result<FileContent, AppError> {
        let text = String::from_utf8(data.to_vec()).map_err(|e| {
            AppError::ExtractionFailed(format!("Invalid UTF-8 in text file: {}", e))
        })?;

        Ok(FileContent::Text(text))
    }
}
