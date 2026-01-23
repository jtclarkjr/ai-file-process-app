use axum::{
    extract::{Multipart, Query, State},
    routing::{get, post},
    Json, Router,
};
use bytes::Bytes;
use std::sync::Arc;
use std::time::Instant;

use crate::errors::AppError;
use crate::models::{
    FileProcessingState, ProcessQuery, ProcessResponse, ProcessResult, ProviderInfo,
    ProvidersResponse, SupportedFileType, SupportedTypeInfo, SupportedTypesResponse,
    TokenUsageResponse,
};
use crate::services::{AiProvider, AiRequest, AiResponse, Operation};

pub fn file_routes() -> Router<Arc<FileProcessingState>> {
    Router::new()
        .route("/process", post(process_file))
        .route("/supported-types", get(get_supported_types))
        .route("/providers", get(get_providers))
}

async fn process_file(
    State(state): State<Arc<FileProcessingState>>,
    Query(query): Query<ProcessQuery>,
    mut multipart: Multipart,
) -> Result<Json<ProcessResponse>, AppError> {
    let start = Instant::now();

    // Parse operation
    let operation: Operation = query.operation.parse()?;

    // Get the appropriate provider
    let provider: &dyn AiProvider = match query.provider.to_lowercase().as_str() {
        "openai" => state
            .openai
            .as_ref()
            .map(|p| p.as_ref())
            .ok_or_else(|| AppError::ProviderNotConfigured("OpenAI".to_string()))?,
        "anthropic" => state
            .anthropic
            .as_ref()
            .map(|p| p.as_ref())
            .ok_or_else(|| AppError::ProviderNotConfigured("Anthropic".to_string()))?,
        other => {
            return Err(AppError::InvalidRequest(format!(
                "Unknown provider: {}. Valid options: openai, anthropic",
                other
            )))
        }
    };

    // Extract file from multipart
    let mut file_data: Option<Bytes> = None;
    let mut file_name: Option<String> = None;
    let mut content_type: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::InvalidRequest(format!("Failed to read multipart: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            file_name = field.file_name().map(|s| s.to_string());
            content_type = field.content_type().map(|s| s.to_string());

            file_data =
                Some(field.bytes().await.map_err(|e| {
                    AppError::InvalidRequest(format!("Failed to read file: {}", e))
                })?);
            break;
        }
    }

    let file_data =
        file_data.ok_or_else(|| AppError::InvalidRequest("No file provided".to_string()))?;

    // Log metadata only (no content)
    tracing::info!(
        file_name = ?file_name,
        content_type = ?content_type,
        size_bytes = file_data.len(),
        provider = provider.name(),
        operation = ?operation,
        "Processing file"
    );

    // Validate and detect file type
    let file_type = state.extractor.validate_and_detect_type(
        &file_data,
        content_type.as_deref(),
        file_name.as_deref(),
    )?;

    // Check if provider supports images when needed
    if file_type.is_image() && !provider.supports_vision() {
        return Err(AppError::AiProviderError(format!(
            "Provider {} does not support image processing",
            provider.name()
        )));
    }

    // Extract content
    let extracted = state.extractor.extract(file_data, &file_type)?;

    // Prepare AI request
    let ai_request = AiRequest {
        content: extracted.content,
        operation,
        custom_prompt: query.custom_prompt,
        file_name: file_name.clone(),
    };

    // Process with AI
    let ai_response: AiResponse = provider.process(ai_request).await?;

    let processing_time_ms = start.elapsed().as_millis() as u64;

    tracing::info!(
        file_name = ?file_name,
        model = %ai_response.model,
        processing_time_ms,
        input_tokens = ?ai_response.usage.as_ref().map(|u| u.input_tokens),
        output_tokens = ?ai_response.usage.as_ref().map(|u| u.output_tokens),
        "File processed successfully"
    );

    Ok(Json(ProcessResponse {
        success: true,
        data: Some(ProcessResult {
            result: ai_response.result,
            model: ai_response.model,
            file_type: extracted.file_type,
            original_size: extracted.original_size,
            processing_time_ms,
            usage: ai_response.usage.map(|u| TokenUsageResponse {
                input_tokens: u.input_tokens,
                output_tokens: u.output_tokens,
            }),
        }),
        error: None,
    }))
}

async fn get_supported_types() -> Json<SupportedTypesResponse> {
    let types = SupportedFileType::all_supported()
        .into_iter()
        .map(|info| SupportedTypeInfo {
            file_type: info.file_type,
            extensions: info.extensions.into_iter().map(String::from).collect(),
            description: info.description.to_string(),
        })
        .collect();

    Json(SupportedTypesResponse {
        success: true,
        data: types,
    })
}

async fn get_providers(State(state): State<Arc<FileProcessingState>>) -> Json<ProvidersResponse> {
    let providers = vec![
        ProviderInfo {
            id: "openai".to_string(),
            name: "GPT".to_string(),
            available: state.openai.is_some(),
            supports_vision: state
                .openai
                .as_ref()
                .map(|p| p.supports_vision())
                .unwrap_or(false),
        },
        ProviderInfo {
            id: "anthropic".to_string(),
            name: "Claude".to_string(),
            available: state.anthropic.is_some(),
            supports_vision: state
                .anthropic
                .as_ref()
                .map(|p| p.supports_vision())
                .unwrap_or(false),
        },
    ];

    Json(ProvidersResponse {
        success: true,
        data: providers,
    })
}
