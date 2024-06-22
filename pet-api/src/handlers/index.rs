use std::sync::Arc;

use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json, Extension};

use serde_json::json;
use sqlx::MySqlPool;
use crate::AppState;

pub async fn index() -> impl IntoResponse {
    const MESSAGE: &str = "Welcome to Paws and Claws API. Server is running!";

    let json_response  = json!({
        "status": StatusCode::OK.as_u16(),
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn health_check(State(data): State<Arc<AppState>>) -> impl IntoResponse {
    let pool: &MySqlPool = &data.db;
    let result = sqlx::query("SELECT 1")
        .execute(pool)
        .await;

    match result {
        Ok(_) => {
            let json_response = json!({
                "status": "ok",
                "message": "Database connection is healthy"
            });
            Json(json_response)
        }
        Err(e) => {
            let json_response = json!({
                "status": "error",
                "message": format!("Failed to connect to the database: {}", e)
            });
            Json(json_response)
        }
    }
}