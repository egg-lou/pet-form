use std::sync::Arc;
use axum::extract::{Path, Query, State};
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

pub async fn get_owners(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) -1) * limit;

    let owner_queries = OwnerQueries::new(Arc::new(data.db.clone()));

    let owners = owner_queries.select_all_owners(limit as i32, offset as i32).await;

    match owners {
        Ok(owners) => {
            let response = json!({
                "status":"success",
                "message":"Owners fetched successfully",
                "data": json!({
                    "owners": owners.into_iter().map(|model| filter_db_record(&model)).collect::<Vec<_>>()
                })
            });

            Ok((StatusCode::OK, Json(response)))
        },
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        ))
    }
}

pub async fn add_owner(
    State(data): State<Arc<AppState>>,
    Json(body): Json<AddOwner>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
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

    match owner_queries.select_owner(owner_id.clone()).await {
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

    pub async fn delete_owner(
        Path(owner_id): Path<String>,
        State(data): State<Arc<AppState>>
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let owner_queries = OwnerQueries::new(Arc::new(data.db.clone()));

        match owner_queries.delete_owner(owner_id).await {
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    let response = json!({
                        "status": "success",
                        "message": "Owner deleted successfully"
                    });

                    Ok((StatusCode::OK, Json(response)))
                } else {
                    Err((
                        StatusCode::NOT_FOUND,
                        Json(json!({"status": "error", "message": "Owner not found"}))
                    ))
                }
            },
            Err(e) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}", e)})),
            ))
        }
    }