use std::sync::Arc;

use axum::{Extension, extract::{Path, Query, State}, http::StatusCode, Json, response::IntoResponse};
use axum::response::Response;
use serde_json::json;
use sqlx::MySqlPool;

use crate::AppState;

pub async fn index() -> impl IntoResponse {

    let response  = json!({
        "message": "Welcome to Paws and Claws API. Server is running!"
    });

    (StatusCode::OK, Json(response))
}

pub async fn health_check(State(data): State<Arc<AppState>>) -> impl IntoResponse {
    let pool: &MySqlPool = &data.db;
    let result = sqlx::query("SELECT 1")
        .execute(pool)
        .await;

    match result {
        Ok(_) => {
            let response = json!({
                "message": "Health check is successful!",
                "db_status": "✅ Database connection is healthy"
            });
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = json!({
                "message": "Database connection error!",
                "db_status": "❌ Database connection is unhealthy"
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}