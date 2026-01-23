use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::{AiProvider, AiRequest, AiResponse, FileContent, TokenUsage};
use crate::errors::AppError;

pub struct OpenAiProvider {
    client: Client,
    api_key: String,
    model: String,
    timeout: Duration,
}

impl OpenAiProvider {
    pub fn new(api_key: String, model: String, timeout_secs: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            api_key,
            model,
            timeout: Duration::from_secs(timeout_secs),
        }
    }
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_completion_tokens: Option<u32>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: MessageContent,
}

#[derive(Serialize)]
#[serde(untagged)]
enum MessageContent {
    Text(String),
    Parts(Vec<ContentPart>),
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize)]
struct ImageUrl {
    url: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
    model: String,
    usage: Option<Usage>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: ApiError,
}

#[derive(Deserialize)]
struct ApiError {
    message: String,
}

#[async_trait]
impl AiProvider for OpenAiProvider {
    fn name(&self) -> &'static str {
        "openai"
    }

    fn supports_vision(&self) -> bool {
        // GPT-4 Vision models support images
        self.model.contains("vision")
            || self.model.contains("gpt-4-turbo")
            || self.model.contains("gpt-4o")
    }

    async fn process(&self, request: AiRequest) -> Result<AiResponse, AppError> {
        let prompt = request
            .operation
            .to_prompt(request.custom_prompt.as_deref());

        let content = match request.content {
            FileContent::Text(text) => {
                let full_prompt = format!(
                    "{}\n\n{}{}",
                    prompt,
                    request
                        .file_name
                        .map(|n| format!("File: {}\n\n", n))
                        .unwrap_or_default(),
                    text
                );
                MessageContent::Text(full_prompt)
            }
            FileContent::Image { data, media_type } => {
                if !self.supports_vision() {
                    return Err(AppError::AiProviderError(format!(
                        "Model {} does not support vision/image processing",
                        self.model
                    )));
                }

                let base64_data = BASE64.encode(&data);
                let data_url = format!("data:{};base64,{}", media_type, base64_data);

                MessageContent::Parts(vec![
                    ContentPart::Text {
                        text: format!(
                            "{}{}",
                            prompt,
                            request
                                .file_name
                                .map(|n| format!("\n\nFile: {}", n))
                                .unwrap_or_default()
                        ),
                    },
                    ContentPart::ImageUrl {
                        image_url: ImageUrl { url: data_url },
                    },
                ])
            }
        };

        let (max_tokens, max_completion_tokens) = if self.model.starts_with("gpt-5") {
            (None, Some(4096))
        } else {
            (Some(4096), None)
        };

        let chat_request = ChatRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content,
            }],
            max_tokens,
            max_completion_tokens,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .timeout(self.timeout)
            .json(&chat_request)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            let error: Result<ErrorResponse, _> = serde_json::from_str(&body);
            let message = error
                .map(|e| e.error.message)
                .unwrap_or_else(|_| format!("OpenAI API error: {}", status));
            return Err(AppError::AiProviderError(message));
        }

        let chat_response: ChatResponse = serde_json::from_str(&body)
            .map_err(|e| AppError::AiProviderError(format!("Failed to parse response: {}", e)))?;

        let result = chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| AppError::AiProviderError("No response from OpenAI".to_string()))?;

        Ok(AiResponse {
            result,
            model: chat_response.model,
            usage: chat_response.usage.map(|u| TokenUsage {
                input_tokens: u.prompt_tokens,
                output_tokens: u.completion_tokens,
            }),
        })
    }
}
