mod config;
mod errors;
mod handlers;
mod logging;
mod models;
mod routes;
mod services;
mod storage;

use actix_web::{App, HttpServer, middleware, web};
use dotenv::dotenv;
use tracing_actix_web::TracingLogger;

use crate::config::AppConfig;
use crate::routes::configure_routes;
use crate::storage::memory::MemoryStorage;
use crate::services::{
    SearchService,
    CatalogService,
    OrderService,
    FulfillmentService,
    ProviderService,
    NetworkRegistryService,
};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file if it exists
    dotenv().ok();
    
    // Load configuration
    let config = AppConfig::new().expect("Failed to load configuration");
    
    // Initialize logging
    logging::init_logging(&config.logging);
    
    tracing::info!("Starting UHI Gateway server on {}:{}", config.server.host, config.server.port);
    
    // Initialize storage (wrapped in Arc for thread-safe reference counting)
    let storage = Arc::new(MemoryStorage::new());
    
    // Initialize services with storage dependency
    let search_service = web::Data::new(SearchService::new(storage.clone()));
    let catalog_service = web::Data::new(CatalogService::new(storage.clone()));
    let order_service = web::Data::new(OrderService::new(storage.clone()));
    let fulfillment_service = web::Data::new(FulfillmentService::new(storage.clone()));
    let provider_service = web::Data::new(ProviderService::new(storage.clone()));
    let network_registry_service = web::Data::new(NetworkRegistryService::new(storage.clone()));
    
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
            // Add services to application state for dependency injection
            .app_data(search_service.clone())
            .app_data(catalog_service.clone())
            .app_data(order_service.clone())
            .app_data(fulfillment_service.clone())
            .app_data(provider_service.clone())
            .app_data(network_registry_service.clone())
            // Configure app state with configuration
            .app_data(web::Data::new(config.clone()))
            // Configure API routes
            .configure(configure_routes)
    })
    .bind(format!("{}:{}", server_host, server_port))?
    .workers(server_workers)
    .run()
    .await
}
