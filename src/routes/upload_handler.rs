use crate::handlers::upload_handler::upload_handler;
use axum::{Router, routing::post};

pub fn upload_routes() -> Router {
    Router::new().route("/upload", post(upload_handler))
}
