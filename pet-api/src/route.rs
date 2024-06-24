use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::handlers::owner_handler::search_owner;
use crate::{
    handlers::{
        index_handler::health_check,
        index_handler::index,
        owner_handler::{add_owner, delete_owner, get_owner_and_pets, get_owners, update_owner},
        pet_handler::{add_pet, delete_pet, get_pets, update_pet},
        vet_handler::{add_vet, delete_vet, get_vet_lists, get_vets, update_vet},
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let owner_routes = Router::new()
        .route("/get_owners", get(get_owners))
        .route("/get_owner_and_pets/:owner_id", get(get_owner_and_pets))
        .route("/search_owner", get(search_owner))
        .route("/add_owner", post(add_owner))
        .route("/update_owner/:owner_id", patch(update_owner))
        .route("/delete_owner/:owner_id", delete(delete_owner));

    let pet_routes = Router::new()
        .route("/get_pets", get(get_pets))
        .route("/add_pet", post(add_pet))
        .route("/update_pet/:pet_id", patch(update_pet))
        .route("/delete_pet/:pet_id", delete(delete_pet));

    let vet_routes = Router::new()
        .route("/get_vets", get(get_vets))
        .route("/add_vet", post(add_vet))
        .route("/update_vet/:vet_id", patch(update_vet))
        .route("/delete_vet/:vet_id", delete(delete_vet))
        .route("/get_vet_lists", get(get_vet_lists));

    Router::new()
        .route("/api", get(index))
        .route("/api/health_check", get(health_check))
        .nest("/api/owner", owner_routes)
        .nest("/api/pet", pet_routes)
        .nest("/api/vet", vet_routes)
        .with_state(app_state)
}
