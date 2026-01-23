use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("File too large: {0} bytes exceeds maximum of {1} bytes")]
    FileTooLarge(usize, usize),

    #[error("Unsupported file type: {0}")]
    UnsupportedFileType(String),

    #[error("MIME type mismatch: declared {declared}, detected {detected}")]
    MimeTypeMismatch { declared: String, detected: String },

    #[error("Decompression bomb detected: ratio {0}:1 exceeds maximum of {1}:1")]
    DecompressionBomb(usize, usize),

    #[error("File extraction failed: {0}")]
    ExtractionFailed(String),

    #[error("AI provider error: {0}")]
    AiProviderError(String),

    #[error("AI provider not configured: {0}")]
    ProviderNotConfigured(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Request timeout")]
    Timeout,

    #[error("Internal error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::FileTooLarge(_, _) => (StatusCode::PAYLOAD_TOO_LARGE, self.to_string()),
            AppError::UnsupportedFileType(_) => {
                (StatusCode::UNSUPPORTED_MEDIA_TYPE, self.to_string())
            }
            AppError::MimeTypeMismatch { .. } => {
                (StatusCode::UNSUPPORTED_MEDIA_TYPE, self.to_string())
            }
            AppError::DecompressionBomb(_, _) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::ExtractionFailed(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            AppError::AiProviderError(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            AppError::ProviderNotConfigured(_) => {
                (StatusCode::SERVICE_UNAVAILABLE, self.to_string())
            }
            AppError::InvalidRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Timeout => (StatusCode::GATEWAY_TIMEOUT, self.to_string()),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        // Log internal errors but don't expose details
        if matches!(self, AppError::Internal(_)) {
            tracing::error!("Internal error: {}", self);
        }

        let body = Json(json!({
            "success": false,
            "error": error_message,
            "data": null
        }));

        (status, body).into_response()
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            AppError::Timeout
        } else {
            AppError::AiProviderError(err.to_string())
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}
