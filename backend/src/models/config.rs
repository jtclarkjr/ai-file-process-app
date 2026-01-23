use std::env;

#[derive(Clone)]
pub struct Config {
    // AI Provider settings
    pub openai_api_key: Option<String>,
    pub openai_model: String,
    pub anthropic_api_key: Option<String>,
    pub anthropic_model: String,

    // Limits
    pub max_file_size_bytes: usize,
    pub request_timeout_secs: u64,
    pub ai_timeout_secs: u64,

    // Decompression bomb protection
    pub max_decompression_ratio: usize,
    pub max_decompressed_size_bytes: usize,
}

impl Config {
    pub fn from_env() -> Self {
        let max_file_size_mb: usize = env::var("MAX_FILE_SIZE_MB")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10);

        Self {
            // AI Providers
            openai_api_key: env::var("OPENAI_API_KEY").ok(),
            openai_model: env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4-turbo".to_string()),
            anthropic_api_key: env::var("ANTHROPIC_API_KEY").ok(),
            anthropic_model: env::var("ANTHROPIC_MODEL")
                .unwrap_or_else(|_| "claude-3-sonnet-20240229".to_string()),

            // Limits
            max_file_size_bytes: max_file_size_mb * 1024 * 1024,
            request_timeout_secs: env::var("REQUEST_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(120),
            ai_timeout_secs: env::var("AI_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(60),

            // Decompression bomb protection
            max_decompression_ratio: 100, // 100:1 max ratio
            max_decompressed_size_bytes: 50 * 1024 * 1024, // 50MB max
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from_env()
    }
}
