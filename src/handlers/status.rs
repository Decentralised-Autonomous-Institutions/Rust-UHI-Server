use actix_web::{web, HttpResponse, Result};
use serde_json::Value;
use crate::models::order::{OrderStatus, OrderStatusRequest, OrderStatusResponse};
use crate::services::order::OrderService;
use tracing::instrument;

#[instrument(skip(payload, service))]
pub async fn status(
    payload: web::Json<OrderStatusRequest>,
    service: web::Data<OrderService>,
) -> Result<HttpResponse> {
    tracing::info!("Received status request for order {}", payload.order_id);
    
    match service.status(&payload.order_id).await {
        Ok(status) => {
            // Get the order with updated status
            match service.get_order(&payload.order_id).await {
                Ok(order) => {
                    let response = OrderStatusResponse { order };
                    Ok(HttpResponse::Ok().json(response))
                }
                Err(err) => {
                    tracing::error!("Failed to get order: {}", err);
                    Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": format!("Failed to get order: {}", err)
                    })))
                }
            }
        }
        Err(err) => {
            tracing::error!("Failed to get order status: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to get order status: {}", err)
            })))
        }
    }
}

#[instrument(skip(payload, service))]
pub async fn on_status(
    payload: web::Json<OrderStatusResponse>,
    service: web::Data<OrderService>,
) -> Result<HttpResponse> {
    tracing::info!("Received on_status request for order {}", payload.order.id);
    
    // Create status object from the order
    let status = OrderStatus {
        state: payload.order.state.clone(),
        updated_at: payload.order.updated_at,
    };
    
    match service.on_status(&payload.order.id, status).await {
        Ok(updated_order) => {
            let response = OrderStatusResponse { order: updated_order };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => {
            tracing::error!("Failed to process status update: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to process status update: {}", err)
            })))
        }
    }
}
