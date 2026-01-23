use bytes::Bytes;

use crate::errors::AppError;
use crate::models::{Config, ExtractedContent, FileContent, FileExtractor, SupportedFileType};

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
