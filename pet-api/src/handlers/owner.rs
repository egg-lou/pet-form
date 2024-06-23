use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use crate::{AppState,
            schemas::{
                owner::{AddOwner, UpdateOwner},
                helper::{FilterOptions}
            },
            models::{
                owner::{OwnerModel, OwnerModelResponse}
            },
            utils::model_to_response::{filter_db_record}
};

pub async fn add_owner(State(data): State<Arc<AppState>>, Json(body): Json<AddOwner> ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let owner_id = uuid::Uuid::new_v4().to_string();

    let query_result =
        sqlx::query(r#"INSERT INTO owner (owner_id, owner_name, owner_email, owner_phone_number, owner_address) VALUES (?, ?, ?, ?, ?)"#)
            .bind(owner_id.clone())
            .bind(body.owner_name.to_string())
            .bind(body.owner_email.to_string())
            .bind(body.owner_phone_number.to_string())
            .bind(body.owner_address.to_string())
            .execute(&data.db)
            .await
            .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = json!({
                "status":"fail",
                "message":"Owner already exists"
            });
            return Err ((StatusCode::CONFLICT, Json(error_response)));
        }
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    let owner: Result<OwnerModel, _> = sqlx::query_as("SELECT * FROM owner WHERE owner_id = ?")
        .bind(owner_id)
        .fetch_one(&data.db)
        .await;

    match owner {
        Ok(owner) => {
            let response = json!({
                "status":"success",
                "message":"Owner added successfully",
                "data": json!({
                    "owner": filter_db_record(&owner)
                })
            });

            Ok((StatusCode::CREATED, Json(response)))
        },
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        ))
    }
}