mod config;
mod errors;
mod handlers;
mod logging;
mod routes;

use actix_web::{App, HttpServer, middleware, web};
use dotenv::dotenv;
use tracing_actix_web::TracingLogger;

use crate::config::AppConfig;
use crate::routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file if it exists
    dotenv().ok();
    
    // Load configuration
    let config = AppConfig::new().expect("Failed to load configuration");
    
    // Initialize logging
    logging::init_logging(&config.logging);
    
    tracing::info!("Starting UHI Gateway server on {}:{}", config.server.host, config.server.port);
    
    // Store config values for the HTTP server
    let server_host = config.server.host.clone();
    let server_port = config.server.port;
    let server_workers = config.server.workers;
    
    // Create and start the HTTP server
    HttpServer::new(move || {
        App::new()
            // Enable logger middleware
            .wrap(TracingLogger::default())
            // Enable body size limits
            .wrap(middleware::NormalizePath::trim())
            // Configure API routes
            .configure(configure_routes)
            // Configure app state with configuration
            .app_data(web::Data::new(config.clone()))
    })
    .bind(format!("{}:{}", server_host, server_port))?
    .workers(server_workers)
    .run()
    .await
}
