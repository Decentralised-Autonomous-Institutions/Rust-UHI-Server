use crate::models::catalog::{SearchRequest, SearchResponse};
use crate::services::{SearchService, ServiceError};
use actix_web::{web, Error, HttpResponse, Result};
use serde_json::Value;
use tracing::instrument;

#[instrument(skip(payload, service))]
pub async fn search(
    payload: web::Json<SearchRequest>,
    service: web::Data<SearchService>,
) -> Result<HttpResponse, Error> {
    tracing::info!("Received search request");

    // Call the service layer to process the search request
    match service.search(payload.into_inner()).await {
        Ok(response) => {
            tracing::info!("Search completed successfully");
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => {
            tracing::error!("Search error: {}", err);
            match err {
                ServiceError::NotFound(msg) => {
                    Ok(HttpResponse::NotFound().json(serde_json::json!({
                        "error": msg
                    })))
                }
                ServiceError::Validation(msg) => {
                    Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": msg
                    })))
                }
                _ => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Internal server error"
                }))),
            }
        }
    }
}

#[instrument(skip(payload, service))]
pub async fn on_search(
    payload: web::Json<SearchResponse>,
    service: web::Data<SearchService>,
) -> Result<HttpResponse, Error> {
    tracing::info!("Received on_search request");

    // Extract provider_id from the payload or request context
    // For now, we'll use a placeholder provider ID
    let provider_id = "provider-123"; // In a real implementation, this would come from the request

    match service.on_search(provider_id, payload.into_inner()).await {
        Ok(_) => {
            tracing::info!("On_search processed successfully");
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "status": "Success"
            })))
        }
        Err(err) => {
            tracing::error!("On_search error: {}", err);
            match err {
                ServiceError::Validation(msg) => {
                    Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": msg
                    })))
                }
                _ => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Internal server error"
                }))),
            }
        }
    }
}
