pub mod file_processing;

use axum::{routing::get, Router};
use std::sync::Arc;

use crate::models::FileProcessingState;
pub use file_processing::file_routes;

pub fn api_routes(state: Arc<FileProcessingState>) -> Router {
    Router::new()
        .route("/health", get(health))
        .nest("/files", file_routes())
        .with_state(state)
}

async fn health() -> &'static str {
    "OK"
}
