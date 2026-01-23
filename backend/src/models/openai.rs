use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct OpenAiProvider {
  pub(crate) client: Client,
  pub(crate) api_key: String,
  pub(crate) model: String,
  pub(crate) timeout: Duration,
}

#[derive(Serialize)]
pub(crate) struct ChatRequest {
  pub(crate) model: String,
  pub(crate) messages: Vec<Message>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) max_tokens: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) max_completion_tokens: Option<u32>,
}

#[derive(Serialize)]
pub(crate) struct Message {
  pub(crate) role: String,
  pub(crate) content: MessageContent,
}

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum MessageContent {
  Text(String),
  Parts(Vec<ContentPart>),
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub(crate) enum ContentPart {
  #[serde(rename = "text")]
  Text { text: String },
  #[serde(rename = "image_url")]
  ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize)]
pub(crate) struct ImageUrl {
  pub(crate) url: String,
}

#[derive(Deserialize)]
pub(crate) struct ChatResponse {
  pub(crate) choices: Vec<Choice>,
  pub(crate) model: String,
  pub(crate) usage: Option<Usage>,
}

#[derive(Deserialize)]
pub(crate) struct Choice {
  pub(crate) message: ResponseMessage,
}

#[derive(Deserialize)]
pub(crate) struct ResponseMessage {
  pub(crate) content: String,
}

#[derive(Deserialize)]
pub(crate) struct Usage {
  pub(crate) prompt_tokens: u32,
  pub(crate) completion_tokens: u32,
}

#[derive(Deserialize)]
pub(crate) struct ErrorResponse {
  pub(crate) error: ApiError,
}

#[derive(Deserialize)]
pub(crate) struct ApiError {
  pub(crate) message: String,
}
