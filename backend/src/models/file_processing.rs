use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::{FileExtractor, SupportedFileType};
use crate::services::AiProvider;

pub struct FileProcessingState {
  pub openai: Option<Arc<dyn AiProvider>>,
  pub anthropic: Option<Arc<dyn AiProvider>>,
  pub extractor: FileExtractor,
}

#[derive(Debug, Deserialize)]
pub struct ProcessQuery {
  pub provider: String,
  pub operation: String,
  pub custom_prompt: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProcessResponse {
  pub success: bool,
  pub data: Option<ProcessResult>,
  pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProcessResult {
  pub result: String,
  pub model: String,
  pub file_type: SupportedFileType,
  pub original_size: usize,
  pub processing_time_ms: u64,
  pub usage: Option<TokenUsageResponse>,
}

#[derive(Debug, Serialize)]
pub struct TokenUsageResponse {
  pub input_tokens: u32,
  pub output_tokens: u32,
}

#[derive(Debug, Serialize)]
pub struct SupportedTypesResponse {
  pub success: bool,
  pub data: Vec<SupportedTypeInfo>,
}

#[derive(Debug, Serialize)]
pub struct SupportedTypeInfo {
  pub file_type: SupportedFileType,
  pub extensions: Vec<String>,
  pub description: String,
}

#[derive(Debug, Serialize)]
pub struct ProvidersResponse {
  pub success: bool,
  pub data: Vec<ProviderInfo>,
}

#[derive(Debug, Serialize)]
pub struct ProviderInfo {
  pub id: String,
  pub name: String,
  pub available: bool,
  pub supports_vision: bool,
}
