use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
  Summarize,
  Extract,
  Analyze,
  Classify,
  Custom,
}

impl Operation {
  pub fn to_prompt(&self, custom_prompt: Option<&str>) -> String {
    match self {
      Operation::Summarize => "Provide a concise summary of the following content. \
                 Focus on the main points and key takeaways."
        .to_string(),
      Operation::Extract => "Extract all important information from the following content. \
                 Include key facts, figures, names, dates, and any structured data. \
                 Format the output as a structured list."
        .to_string(),
      Operation::Analyze => "Analyze the following content in depth. \
                 Identify themes, patterns, sentiment, and provide insights. \
                 Include both objective observations and interpretive analysis."
        .to_string(),
      Operation::Classify => "Classify the following content. Determine:\n\
                 1. Document type (e.g., report, letter, article, code, etc.)\n\
                 2. Primary topic/subject\n\
                 3. Target audience\n\
                 4. Tone (formal, informal, technical, etc.)\n\
                 5. Key categories or tags that apply"
        .to_string(),
      Operation::Custom => custom_prompt.unwrap_or("Process this content.").to_string(),
    }
  }
}

impl std::str::FromStr for Operation {
  type Err = AppError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "summarize" => Ok(Operation::Summarize),
      "extract" => Ok(Operation::Extract),
      "analyze" => Ok(Operation::Analyze),
      "classify" => Ok(Operation::Classify),
      "custom" => Ok(Operation::Custom),
      _ => Err(AppError::InvalidRequest(format!(
        "Invalid operation: {}. Valid options: summarize, extract, analyze, classify, custom",
        s
      ))),
    }
  }
}

#[derive(Debug, Clone)]
pub enum FileContent {
  Text(String),
  Image { data: Vec<u8>, media_type: String },
}

#[derive(Debug)]
pub struct AiRequest {
  pub content: FileContent,
  pub operation: Operation,
  pub custom_prompt: Option<String>,
  pub file_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
  pub input_tokens: u32,
  pub output_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiResponse {
  pub result: String,
  pub model: String,
  pub usage: Option<TokenUsage>,
}
