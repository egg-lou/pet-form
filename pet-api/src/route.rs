use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post, patch},
};

use crate::{
    AppState,
    handlers::{
        index::health_check,
        index::index,
        owner::{
            add_owner,
            delete_owner,
            get_owners,
            update_owner
        },
    }
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let owner_routes = Router::new()
        .route("/get_owners", get(get_owners))
        .route("/add_owner", post(add_owner))
        .route("/update_owner/:owner_id", patch(update_owner))
        .route("/delete_owner/:owner_id", delete(delete_owner));

    Router::new()
        .route("/api", get(index))
        .route("/api/health_check", get(health_check))
        .nest("/api/owner", owner_routes)
        .with_state(app_state)
}