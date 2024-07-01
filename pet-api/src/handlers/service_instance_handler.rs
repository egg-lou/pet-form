use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

use crate::AppState;
use crate::db::queries::service_instance_queries::ServiceInstanceQueries;
use crate::schemas::helper_schema::FilterOptions;
use crate::schemas::service_instance_schema::AddServiceInstance;


pub async fn add_service_instance(
    State(data): State<Arc<AppState>>,
    Json(body): Json<AddServiceInstance>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service_instance_id = uuid::Uuid::new_v4().to_string();
    let service_instance_queries = ServiceInstanceQueries::new(Arc::new(data.db.clone()));
    let current_date = chrono::Local::now().date_naive().to_string();
    match service_instance_queries
        .insert_service_instance(
            service_instance_id.clone(),
            current_date,
            body.service_type.clone(),
            body.service_reason,
            body.general_diagnosis,
            body.requires_followup,
            body.followup_date,
            body.pet_id.clone(),
            body.grooming_type,
            body.preventive_care,
            body.surgery,
        )
        .await
    {
        Ok(service_instance) => Ok((StatusCode::CREATED, Json(service_instance))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

pub async fn get_pet_histories(
    Path(pet_id): Path<String>,
    State(data): State<Arc<AppState>>,
    opts: Option<Query<FilterOptions>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;
    let service_instance_queries = ServiceInstanceQueries::new(Arc::new(data.db.clone()));
    match service_instance_queries
        .get_services_history_of_pet(limit as i32, offset as i32, pet_id)
        .await
    {
        Ok(service_instance) => Ok((StatusCode::OK, Json(service_instance))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}
