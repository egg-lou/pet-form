use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::utils::validator::validate_field;
use crate::{
    db::queries::owner_queries::OwnerQueries,
    schemas::{
        helper_schema::FilterOptions,
        owner_schema::{AddOwner, UpdateOwner},
    },
    utils::{
        handle_duplicate_error::handle_duplicate_entry_error, model_to_response::filter_db_record,
    },
    AppState,
};

pub async fn get_owners(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let owner_queries = OwnerQueries::new(Arc::new(data.db.clone()));
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;
    let search = opts.search.clone();
    let total_owners = owner_queries
        .count_all_owners(search.clone())
        .await
        .unwrap_or_default();
    let total_pages = (total_owners as f64 / limit as f64).ceil() as i32;

    let owners = owner_queries
        .select_all_owners(limit as i32, offset as i32, search)
        .await;

    match owners {
        Ok(owners) => {
            let response = json!({
                "status":"success",
                "message":"Owners fetched successfully",
                "owners": owners.into_iter().map(|model| filter_db_record(&model))
                .collect::<Vec<_>>(),
                "total_pages": total_pages,
            });

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

pub async fn get_owner_and_pets(
    State(data): State<Arc<AppState>>,
    Path(owner_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let owner_queries = OwnerQueries::new(Arc::new(data.db.clone()));

    match owner_queries.get_owner_and_pets(owner_id.clone()).await {
        Ok(owner_with_pets) => {
            let response = json!({
                "status":"success",
                "message":"Owner and pets fetched successfully",
                    "owner": filter_db_record(&owner_with_pets.owner),
                    "pets": owner_with_pets.pets.into_iter().map(|model| filter_db_record(&model)).collect::<Vec<_>>()
            });

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

pub async fn add_owner(
    State(data): State<Arc<AppState>>,
    Json(body): Json<AddOwner>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let owner_id = uuid::Uuid::new_v4().to_string();

    let owner_queries = OwnerQueries::new(Arc::new(data.db.clone()));

    let query_result = owner_queries
        .insert_owner(
            owner_id.clone(),
            body.owner_name.to_string(),
            body.owner_email.to_string(),
            body.owner_phone_number.to_string(),
            body.owner_address.to_string(),
        )
        .await;

    if let Err(err) = query_result {
        return handle_duplicate_entry_error(err, "Owner");
    }

    match owner_queries.select_owner(owner_id.clone()).await {
        Ok(owner) => {
            let response = json!({
                "status":"success",
                "message":"Owner added successfully",
                    "owner": filter_db_record(&owner),
            });

            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

pub async fn update_owner(
    Path(owner_id): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateOwner>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if let Err(message) = validate_owner_fields(&body) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"status": "error", "message": message})),
        ));
    }

    let owner_queries = OwnerQueries::new(Arc::new(data.db.clone()));

    let owner = owner_queries.select_owner(owner_id.clone()).await;

    match owner {
        Ok(_) => {
            let owner_name = body.owner_name.clone();
            let owner_email = body.owner_email.clone();
            let owner_phone_number = body.owner_phone_number.clone();
            let owner_address = body.owner_address.clone();

            let query_result = owner_queries
                .update_owner(
                    owner_id.clone(),
                    owner_name,
                    owner_email,
                    owner_phone_number,
                    owner_address,
                )
                .await;

            if let Err(err) = query_result {
                return handle_duplicate_entry_error(err, "Owner");
            }

            match owner_queries.select_owner(owner_id.clone()).await {
                Ok(owner) => {
                    let response = json!({
                        "status":"success",
                        "message":"Owner updated successfully",
                            "owner": filter_db_record(&owner)
                    });

                    Ok((StatusCode::OK, Json(response)))
                }
                Err(e) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"status": "error", "message": format!("{:?}", e)})),
                )),
            }
        }
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

pub async fn delete_owner(
    Path(owner_id): Path<String>,
    State(data): State<Arc<AppState>>,
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
                    Json(json!({"status": "error", "message": "Owner not found"})),
                ))
            }
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

fn validate_owner_fields(body: &UpdateOwner) -> Result<(), String> {
    validate_field(&body.owner_name, "Owner name cannot be empty")?;
    validate_field(&body.owner_email, "Owner email cannot be empty")?;
    validate_field(
        &body.owner_phone_number,
        "Owner phone number cannot be empty",
    )?;
    validate_field(&body.owner_address, "Owner address cannot be empty")?;

    Ok(())
}
