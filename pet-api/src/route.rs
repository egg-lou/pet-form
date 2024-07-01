use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{
    AppState,
    handlers::{
        index_handler::health_check,
        index_handler::index,
        owner_handler::{add_owner, delete_owner, get_owner_and_pets, get_owners, update_owner},
        pet_handler::{add_pet, delete_pet, get_pets, update_pet},
        service_instance_handler::{
            add_grooming_to_instance, add_preventive_care_to_instance, add_service_instance,
            add_surgery_to_instance, delete_grooming_from_instance,
            delete_preventive_care_from_instance, delete_service, delete_surgery_from_instance,
            get_pet_histories, get_specific_service_instance, update_service_instance,
            update_surgery_from_instance,
        },
        vet_handler::{add_vet, delete_vet, get_vet_lists, get_vets, update_vet},
    },
};
use crate::handlers::owner_handler::search_owner;


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

    let service_instance_routes = Router::new()
        .route("/add_service_instance", post(add_service_instance))
        .route("/get_pet_histories/:pet_id", get(get_pet_histories))
        .route(
            "/get_specific_service_instance/:service_instance_id",
            get(get_specific_service_instance),
        )
        .route(
            "/update_service_instance/:service_instance_id",
            patch(update_service_instance),
        )
        .route("/add_surgery_to_instance", post(add_surgery_to_instance))
        .route("/add_grooming_to_instance", post(add_grooming_to_instance))
        .route(
            "/add_preventive_care_to_instance",
            post(add_preventive_care_to_instance),
        )
        .route(
            "/update_surgery_from_instance",
            patch(update_surgery_from_instance),
        )
        .route("/delete_service/:service_id", delete(delete_service))
        .route(
            "/delete_grooming_from_instance",
            delete(delete_grooming_from_instance),
        )
        .route(
            "/delete_preventive_care_from_instance",
            delete(delete_preventive_care_from_instance),
        )
        .route(
            "/delete_surgery_from_instance",
            delete(delete_surgery_from_instance),
        );

    Router::new()
        .route("/api", get(index))
        .route("/api/health_check", get(health_check))
        .nest("/api/owner", owner_routes)
        .nest("/api/pet", pet_routes)
        .nest("/api/vet", vet_routes)
        .nest("/api/service_instance", service_instance_routes)
        .with_state(app_state)
}
