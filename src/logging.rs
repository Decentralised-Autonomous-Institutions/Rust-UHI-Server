use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, fmt};
use crate::config::LoggingConfig;

pub fn init_logging(config: &LoggingConfig) {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            EnvFilter::new(format!("rust_uhi={},actix_web=info,sqlx=warn", config.level))
        });

    match config.format.as_str() {
        "json" => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(fmt::layer().json())
                .init();
        },
        _ => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(fmt::layer().pretty())
                .init();
        }
    }

    tracing::info!("Logging initialized with level: {}", config.level);
}

// Create a span for request tracing
pub fn create_request_span(transaction_id: &str, message_id: &str) -> tracing::Span {
    tracing::info_span!(
        "request",
        transaction_id = %transaction_id,
        message_id = %message_id,
    )
}

// Helper for structured logging
pub fn log_error<E: std::fmt::Display>(error: &E, context: &str) {
    tracing::error!(
        error = %error,
        context = %context,
        "An error occurred"
    );
} 