use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    handlers::{
        index::health_check,
        index::index,
        owner::{add_owner, delete_owner, get_owner_and_pets, get_owners, update_owner},
        pet::{add_pet, get_pets},
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let owner_routes = Router::new()
        .route("/get_owners", get(get_owners))
        .route("/get_owner_and_pets/:owner_id", get(get_owner_and_pets))
        .route("/add_owner", post(add_owner))
        .route("/update_owner/:owner_id", patch(update_owner))
        .route("/delete_owner/:owner_id", delete(delete_owner));

    let pet_routes = Router::new()
        .route("/get_pets", get(get_pets))
        .route("/add_pet", post(add_pet));

    Router::new()
        .route("/api", get(index))
        .route("/api/health_check", get(health_check))
        .nest("/api/owner", owner_routes)
        .nest("/api/pet", pet_routes)
        .with_state(app_state)
}
