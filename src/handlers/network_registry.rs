use actix_web::{web, HttpResponse, Result};
use tracing::instrument;

use crate::models::network_registry::{LookupRequest, LookupResponse};
use crate::services::NetworkRegistryService;
use crate::errors::AppError;

/// Handle lookup requests for network registry participants
#[instrument(skip(service, payload))]
pub async fn lookup(
    service: web::Data<NetworkRegistryService>,
    payload: web::Json<LookupRequest>,
) -> Result<HttpResponse, AppError> {
    tracing::info!("Received network registry lookup request");
    
    // Call the service to perform the lookup
    let response = service.lookup_participants(payload.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(response))
}

/// Handle signature validation requests
#[instrument(skip(service, payload))]
pub async fn validate_signature(
    service: web::Data<NetworkRegistryService>,
    payload: web::Json<ValidateSignatureRequest>,
) -> Result<HttpResponse, AppError> {
    tracing::info!("Received signature validation request");
    
    let request = payload.into_inner();
    
    // Call the service to validate the signature
    let is_valid = service
        .validate_signature(
            &request.subscriber_id,
            &request.signature,
            request.message.as_bytes(),
        )
        .await?;
    
    Ok(HttpResponse::Ok().json(ValidateSignatureResponse { valid: is_valid }))
}

/// Request for signature validation
#[derive(serde::Deserialize)]
pub struct ValidateSignatureRequest {
    /// Subscriber ID whose public key will be used
    pub subscriber_id: String,
    /// Signature to validate
    pub signature: String,
    /// Original message that was signed
    pub message: String,
}

/// Response from signature validation
#[derive(serde::Serialize)]
pub struct ValidateSignatureResponse {
    /// Whether the signature is valid
    pub valid: bool,
}
