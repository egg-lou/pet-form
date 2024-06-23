use std::sync::Arc;

use axum::{
    routing::{get, post, delete, put},
    Router
};
use crate::{
    handlers:: {
        index::index,
        index::health_check,
        owner::add_owner,
    },
    AppState
};


pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api", get(index))
        .route("/api/health_check", get(health_check))
        .route("/api/owner/add_owner", post(add_owner))
        .with_state(app_state)
}