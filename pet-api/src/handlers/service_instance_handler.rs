use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

use crate::AppState;
use crate::db::queries::service_instance_queries::ServiceInstanceQueries;
use crate::schemas::helper_schema::FilterOptions;
use crate::schemas::service_instance_schema::{
    AddPreventiveCareToExisting, AddServiceInstance, AddSurgery, UpdateServiceInstance,
    UpdateSurgery,
};


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
pub async fn get_specific_service_instance(
    Path(service_instance_id): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service_instance_queries = ServiceInstanceQueries::new(Arc::new(data.db.clone()));
    match service_instance_queries
        .get_specific_instance(service_instance_id)
        .await
    {
        Ok(service_instance) => Ok((StatusCode::OK, Json(service_instance))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

pub async fn update_service_instance(
    Path(service_instance_id): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateServiceInstance>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service_instance_queries = ServiceInstanceQueries::new(Arc::new(data.db.clone()));
    match service_instance_queries
        .update_service_instance(body, service_instance_id)
        .await
    {
        Ok(service_instance) => Ok((StatusCode::OK, Json(service_instance))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

pub async fn delete_service(
    Path(service_instance_id): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service_instance_queries = ServiceInstanceQueries::new(Arc::new(data.db.clone()));
    match service_instance_queries
        .delete_service_instance(service_instance_id)
        .await
    {
        Ok(service_instance) => Ok((StatusCode::OK, Json(service_instance))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

pub async fn delete_grooming_from_instance(
    Path(grooming_id): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service_instance_queries = ServiceInstanceQueries::new(Arc::new(data.db.clone()));
    let grooming_id = grooming_id.parse::<i32>().map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Invalid grooming_id: {}", e) })),
        )
    })?;
    match service_instance_queries.delete_grooming(grooming_id).await {
        Ok(service_instance) => Ok((StatusCode::OK, Json(service_instance))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

pub async fn delete_preventive_care_from_instance(
    Path(preventive_care_id): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service_instance_queries = ServiceInstanceQueries::new(Arc::new(data.db.clone()));
    let preventive_care_id = preventive_care_id.parse::<i32>().map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Invalid preventive_care_id: {}", e) })),
        )
    })?;
    match service_instance_queries
        .delete_preventive_care(preventive_care_id)
        .await
    {
        Ok(service_instance) => Ok((StatusCode::OK, Json(service_instance))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

pub async fn delete_surgery_from_instance(
    Path(surgery_id): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service_instance_queries = ServiceInstanceQueries::new(Arc::new(data.db.clone()));
    let surgery_id = surgery_id.parse::<i32>().map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Invalid surgery_id: {}", e) })),
        )
    })?;
    match service_instance_queries.delete_surgery(surgery_id).await {
        Ok(service_instance) => Ok((StatusCode::OK, Json(service_instance))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

pub async fn update_surgery_from_instance(
    Path(surgery_id): Path<String>, // change from i32 to String
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateSurgery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service_instance_queries = ServiceInstanceQueries::new(Arc::new(data.db.clone()));
    let surgery_id = surgery_id.parse::<i32>().map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Invalid surgery_id: {}", e) })),
        )
    })?;
    match service_instance_queries
        .update_surgery(body, surgery_id)
        .await
    {
        Ok(service_instance) => Ok((StatusCode::OK, Json(service_instance))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

pub async fn add_preventive_care_to_instance(
    Path(service_instance_id): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<AddPreventiveCareToExisting>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service_instance_queries = ServiceInstanceQueries::new(Arc::new(data.db.clone()));
    match service_instance_queries
        .add_preventive_care(body, service_instance_id)
        .await
    {
        Ok(service_instance) => Ok((StatusCode::CREATED, Json(service_instance))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

pub async fn add_grooming_to_instance(
    Path(service_instance_id): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<Vec<String>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service_instance_queries = ServiceInstanceQueries::new(Arc::new(data.db.clone()));
    match service_instance_queries
        .add_grooming(service_instance_id, body)
        .await
    {
        Ok(service_instance) => Ok((StatusCode::CREATED, Json(service_instance))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

pub async fn add_surgery_to_instance(
    Path(service_instance_id): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<AddSurgery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service_instance_queries = ServiceInstanceQueries::new(Arc::new(data.db.clone()));
    match service_instance_queries
        .add_surgery(body, service_instance_id)
        .await
    {
        Ok(service_instance) => Ok((StatusCode::CREATED, Json(service_instance))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}
