use axum::http::StatusCode;
use axum::Json;
use serde_json::json;

pub(crate) fn handle_duplicate_entry_error(err: sqlx::Error, entity: &str) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    let err_msg = err.to_string();
    if err_msg.contains("Duplicate entry") {
        let error_response = json!({
            "status":"fail",
            "message": format!("{} already exists", entity)
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }
    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"status": "error","message": format!("{:?}", err_msg)})),
    ))
}