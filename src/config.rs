use std::env;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let env = env::var("RUN_ENV").unwrap_or_else(|_| "development".into());
        let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config".into());

        let config = Config::builder()
            // Start with default settings
            .add_source(File::with_name(&format!("{}/default", config_path)).required(false))
            // Add environment-specific settings
            .add_source(File::with_name(&format!("{}/{}", config_path, env)).required(false))
            // Add local settings (excluded from version control)
            .add_source(File::with_name(&format!("{}/local", config_path)).required(false))
            // Override with environment variables (UHI_SERVER__PORT, UHI_DATABASE__URL, etc)
            .add_source(Environment::with_prefix("UHI").separator("__"))
            .build()?;

        config.try_deserialize()
    }
} 