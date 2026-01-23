use async_trait::async_trait;

use crate::errors::AppError;
use crate::models::{AiRequest, AiResponse};

#[async_trait]
pub trait AiProvider: Send + Sync {
  fn name(&self) -> &'static str;
  fn supports_vision(&self) -> bool;
  async fn process(&self, request: AiRequest) -> Result<AiResponse, AppError>;
}
