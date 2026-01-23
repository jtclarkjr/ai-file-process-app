use serde::{Deserialize, Serialize};

use super::{Config, FileContent};

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
      "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => Some(Self::Docx),
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
          "py", "js", "ts", "tsx", "rs", "c", "cpp", "java", "go", "json", "yaml", "yml", "html",
          "css", "xml", "sh", "sql", "rb", "php", "swift", "kt", "scala", "toml",
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
  pub(crate) config: Config,
}
