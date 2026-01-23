use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct AnthropicProvider {
  pub(crate) client: Client,
  pub(crate) api_key: String,
  pub(crate) model: String,
  pub(crate) timeout: Duration,
}

#[derive(Serialize)]
pub(crate) struct MessagesRequest {
  pub(crate) model: String,
  pub(crate) max_tokens: u32,
  pub(crate) messages: Vec<Message>,
}

#[derive(Serialize)]
pub(crate) struct Message {
  pub(crate) role: String,
  pub(crate) content: Vec<ContentBlock>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub(crate) enum ContentBlock {
  #[serde(rename = "text")]
  Text { text: String },
  #[serde(rename = "image")]
  Image { source: ImageSource },
}

#[derive(Serialize)]
pub(crate) struct ImageSource {
  #[serde(rename = "type")]
  pub(crate) source_type: String,
  pub(crate) media_type: String,
  pub(crate) data: String,
}

#[derive(Deserialize)]
pub(crate) struct MessagesResponse {
  pub(crate) content: Vec<ResponseContent>,
  pub(crate) model: String,
  pub(crate) usage: Usage,
}

#[derive(Deserialize)]
pub(crate) struct ResponseContent {
  #[serde(rename = "type")]
  pub(crate) content_type: String,
  pub(crate) text: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct Usage {
  pub(crate) input_tokens: u32,
  pub(crate) output_tokens: u32,
}

#[derive(Deserialize)]
pub(crate) struct ErrorResponse {
  pub(crate) error: ApiError,
}

#[derive(Deserialize)]
pub(crate) struct ApiError {
  pub(crate) message: String,
}
