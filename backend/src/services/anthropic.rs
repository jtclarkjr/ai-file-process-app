use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use reqwest::Client;
use std::time::Duration;

use super::AiProvider;
use crate::errors::AppError;
use crate::models::anthropic::{
    AnthropicProvider, ContentBlock, ErrorResponse, ImageSource, Message, MessagesRequest,
    MessagesResponse,
};
use crate::models::{AiRequest, AiResponse, FileContent, TokenUsage};

impl AnthropicProvider {
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

#[async_trait]
impl AiProvider for AnthropicProvider {
    fn name(&self) -> &'static str {
        "anthropic"
    }

    fn supports_vision(&self) -> bool {
        // Claude 3+ models support vision
        self.model.contains("claude-3")
    }

    async fn process(&self, request: AiRequest) -> Result<AiResponse, AppError> {
        let prompt = request
            .operation
            .to_prompt(request.custom_prompt.as_deref());

        let content_blocks = match request.content {
            FileContent::Text(text) => {
                let full_text = format!(
                    "{}\n\n{}{}",
                    prompt,
                    request
                        .file_name
                        .map(|n| format!("File: {}\n\n", n))
                        .unwrap_or_default(),
                    text
                );
                vec![ContentBlock::Text { text: full_text }]
            }
            FileContent::Image { data, media_type } => {
                if !self.supports_vision() {
                    return Err(AppError::AiProviderError(format!(
                        "Model {} does not support vision/image processing",
                        self.model
                    )));
                }

                let base64_data = BASE64.encode(&data);

                vec![
                    ContentBlock::Text {
                        text: format!(
                            "{}{}",
                            prompt,
                            request
                                .file_name
                                .map(|n| format!("\n\nFile: {}", n))
                                .unwrap_or_default()
                        ),
                    },
                    ContentBlock::Image {
                        source: ImageSource {
                            source_type: "base64".to_string(),
                            media_type,
                            data: base64_data,
                        },
                    },
                ]
            }
        };

        let messages_request = MessagesRequest {
            model: self.model.clone(),
            max_tokens: 4096,
            messages: vec![Message {
                role: "user".to_string(),
                content: content_blocks,
            }],
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .timeout(self.timeout)
            .json(&messages_request)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            let error: Result<ErrorResponse, _> = serde_json::from_str(&body);
            let message = error
                .map(|e| e.error.message)
                .unwrap_or_else(|_| format!("Anthropic API error: {}", status));
            return Err(AppError::AiProviderError(message));
        }

        let messages_response: MessagesResponse = serde_json::from_str(&body)
            .map_err(|e| AppError::AiProviderError(format!("Failed to parse response: {}", e)))?;

        let result = messages_response
            .content
            .iter()
            .filter(|c| c.content_type == "text")
            .filter_map(|c| c.text.clone())
            .collect::<Vec<_>>()
            .join("\n");

        if result.is_empty() {
            return Err(AppError::AiProviderError(
                "No text response from Anthropic".to_string(),
            ));
        }

        Ok(AiResponse {
            result,
            model: messages_response.model,
            usage: Some(TokenUsage {
                input_tokens: messages_response.usage.input_tokens,
                output_tokens: messages_response.usage.output_tokens,
            }),
        })
    }
}
