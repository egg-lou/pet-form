use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

use crate::AppState;
use crate::db::queries::service_instance_queries::ServiceInstanceQueries;
use crate::schemas::service_instance_schema::{AddServiceInstance, InsertPreventiveCare};


pub async fn add_service_instance(
    State(data): State<Arc<AppState>>,
    Json(body): Json<AddServiceInstance>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service_instance_id = uuid::Uuid::new_v4().to_string();
    let service_instance_queries = ServiceInstanceQueries::new(Arc::new(data.db.clone()));
    let current_date = chrono::Local::now().date_naive().to_string();
    let service_types = body.service_type
    match service_instance_queries
        .insert_service_instance(
            service_instance_id.clone(),
            body.service_type,
            current_date,
            body.service_reason,
            body.general_diagnosis,
            body.requires_followup,
            body.followup_date,
            body.pet_id.clone(),
            body.grooming_type,
            body.treatment.map(|treatments| InsertPreventiveCare {
                treatment: treatments,
                vet_id: body.pet_id.clone(),
                service_instance_id: service_instance_id.clone(),
            }),
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
