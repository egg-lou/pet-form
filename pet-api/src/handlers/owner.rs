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
            utils::{
                model_to_response::{filter_db_record},
                handle_duplicate_error::{handle_duplicate_entry_error}
            },
            db::queries::owner_queries::{OwnerQueries}
};

pub async fn add_owner(State(data): State<Arc<AppState>>, Json(body): Json<AddOwner> ) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    let owner_id = uuid::Uuid::new_v4().to_string();

    let owner_queries = OwnerQueries::new(Arc::new(data.db.clone()));

    let query_result = owner_queries.insert_owner(
        owner_id.clone(),
        body.owner_name.to_string(),
        body.owner_email.to_string(),
        body.owner_phone_number.to_string(),
        body.owner_address.to_string()
    ).await;

    if let Err(err) = query_result {
        return handle_duplicate_entry_error(err, "Owner")
    }

    let owner = owner_queries.select_owner(owner_id.clone())
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