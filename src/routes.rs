use crate::handlers::{
    confirm::{confirm, on_confirm},
    init::{init, on_init},
    network_registry::lookup,
    search::{on_search, search},
    select::{on_select, select},
    status::{on_status, status},
};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Search endpoints
            .route("/search", web::post().to(search))
            .route("/on_search", web::post().to(on_search))
            // Select endpoints
            .route("/select", web::post().to(select))
            .route("/on_select", web::post().to(on_select))
            // Init endpoints
            .route("/init", web::post().to(init))
            .route("/on_init", web::post().to(on_init))
            // Confirm endpoints
            .route("/confirm", web::post().to(confirm))
            .route("/on_confirm", web::post().to(on_confirm))
            // Status endpoints
            .route("/status", web::post().to(status))
            .route("/on_status", web::post().to(on_status))
            // Network registry endpoints
            .route("/networkregistry/lookup", web::post().to(lookup)),
    );
}
