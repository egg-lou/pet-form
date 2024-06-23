use std::sync::Arc;

use axum::{
    routing::{get, post, delete, put, },
    Router,
};
use crate::{
    handlers:: {
        index::index,
        index::health_check,
        owner::{
            get_owners,
            delete_owner,
            add_owner
        },
    },
    AppState
};


pub fn create_router(app_state: Arc<AppState>) -> Router {
    let owner_routes = Router::new()
        .route("/get_owners", get(get_owners))
        .route("/add_owner", post(add_owner))
        .route("/delete_owner/:owner_id", delete(delete_owner));

    Router::new()
        .route("/api", get(index))
        .route("/api/health_check", get(health_check))
        .nest("/api/owner", owner_routes)
        .with_state(app_state)
}