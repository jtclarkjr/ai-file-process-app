mod config;
mod errors;
mod routes;
mod services;

use axum::Router;
use std::sync::Arc;
use std::{env, net::SocketAddr, path::PathBuf};
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;
use routes::FileProcessingState;
use services::{AnthropicProvider, FileExtractor, OpenAiProvider};

#[tokio::main]
async fn main() {
    // Load .env file if present
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_timer(UtcTime::rfc_3339()),
        )
        .init();

    // Load configuration
    let config = Config::from_env();

    // Initialize AI providers
    let openai = config.openai_api_key.as_ref().map(|key| {
        tracing::info!(
            "OpenAI provider configured with model: {}",
            config.openai_model
        );
        Arc::new(OpenAiProvider::new(
            key.clone(),
            config.openai_model.clone(),
            config.ai_timeout_secs,
        )) as Arc<dyn services::AiProvider>
    });

    let anthropic = config.anthropic_api_key.as_ref().map(|key| {
        tracing::info!(
            "Anthropic provider configured with model: {}",
            config.anthropic_model
        );
        Arc::new(AnthropicProvider::new(
            key.clone(),
            config.anthropic_model.clone(),
            config.ai_timeout_secs,
        )) as Arc<dyn services::AiProvider>
    });

    if openai.is_none() && anthropic.is_none() {
        tracing::warn!("No AI providers configured. Set OPENAI_API_KEY or ANTHROPIC_API_KEY.");
    }

    // Create file processing state
    let file_state = Arc::new(FileProcessingState {
        openai,
        anthropic,
        extractor: FileExtractor::new(config.clone()),
    });

    // CORS configuration for development
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build API router
    let api_router = Router::new()
        .nest("/api", routes::api_routes(file_state))
        .layer(RequestBodyLimitLayer::new(config.max_file_size_bytes))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Check if static files directory exists (production mode)
    let static_dir = PathBuf::from("static");
    let app = if static_dir.exists() {
        tracing::info!("Serving static files from ./static");
        let serve_dir = ServeDir::new(&static_dir)
            .not_found_service(ServeFile::new(static_dir.join("index.html")));

        api_router.fallback_service(serve_dir)
    } else {
        tracing::info!("No static directory found, running API only");
        api_router
    };

    // Start server
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::new(host.parse().expect("Invalid HOST"), port);
    tracing::info!("Server listening on {}", addr);
    tracing::info!(
        "File processing: max_size={}MB, request_timeout={}s, ai_timeout={}s",
        config.max_file_size_bytes / 1024 / 1024,
        config.request_timeout_secs,
        config.ai_timeout_secs
    );

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
