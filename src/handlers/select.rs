use actix_web::{web, HttpResponse, Result};
use serde_json::Value;
use tracing::instrument;

#[instrument(skip(payload))]
pub async fn select(payload: web::Json<Value>) -> Result<HttpResponse> {
    // Placeholder implementation
    tracing::info!("Received select request");
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Select endpoint - Placeholder response"
    })))
}

#[instrument(skip(payload))]
pub async fn on_select(payload: web::Json<Value>) -> Result<HttpResponse> {
    // Placeholder implementation
    tracing::info!("Received on_select request");
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "On_Select endpoint - Placeholder response"
    })))
} 