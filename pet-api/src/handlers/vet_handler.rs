use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::db::queries::vet_queries::VetQueries;
use crate::schemas::helper_schema::FilterOptions;
use crate::schemas::vet_schema::{AddVet, UpdateVet};
use crate::utils::handle_duplicate_error::handle_duplicate_entry_error;
use crate::utils::model_to_response::filter_db_record;
use crate::utils::validator::validate_field;
use crate::AppState;

pub async fn get_vets(
    State(data): State<Arc<AppState>>,
    opts: Option<Query<FilterOptions>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let vet_queries = VetQueries::new(Arc::new(data.db.clone()));

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let vets = vet_queries
        .select_all_vets(limit as i32, offset as i32)
        .await;

    match vets {
        Ok(vets) => {
            let response = json!({
                "status":"success",
                "message":"Vets fetched successfully",
                "data": json!({
                    "vets": vets.into_iter().map(|model| filter_db_record(&model)).collect::<Vec<_>>()
                })
            });
            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

pub async fn get_vet_lists(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let vet_queries = VetQueries::new(Arc::new(data.db.clone()));

    let vets = vet_queries.vet_lists().await;

    match vets {
        Ok(vets) => {
            let response = json!({
                "status":"success",
                "message":"Vets fetched successfully",
                "data": json!({
                    "vets": vets.into_iter().collect::<Vec<_>>()
                })
            });
            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

pub async fn add_vet(
    State(data): State<Arc<AppState>>,
    Json(body): Json<AddVet>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let vet_id = uuid::Uuid::new_v4().to_string();
    let vet_queries = VetQueries::new(Arc::new(data.db.clone()));

    let _vet = vet_queries
        .insert_vet(
            vet_id.clone(),
            body.vet_name.to_string(),
            body.vet_email.to_string(),
            body.vet_phone_number.to_string(),
            body.vet_license_number.to_string(),
        )
        .await;

    match vet_queries.select_vet(vet_id.clone()).await {
        Ok(vet) => {
            let response = json!({
                "status":"success",
                "message":"Vet added successfully",
                "data": json!({
                    "vet": filter_db_record(&vet)
                })
            });
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

pub async fn delete_vet(
    Path(vet_id): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let vet_queries = VetQueries::new(Arc::new(data.db.clone()));
    match vet_queries.delete_vet(vet_id.clone()).await {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                let response = json!({
                    "status":"success",
                    "message":"Vet deleted successfully",
                });
                Ok((StatusCode::OK, Json(response)))
            } else {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(json!({"status": "error", "message": "Vet not found"})),
                ))
            }
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

pub async fn update_vet(
    Path(vet_id): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateVet>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if let Err(message) = validate_vet_fields(&body) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"status": "error", "message": message})),
        ));
    }

    let vet_queries = VetQueries::new(Arc::new(data.db.clone()));
    let vet = vet_queries.select_vet(vet_id.clone()).await;

    match vet {
        Ok(_) => {
            let vet_name = body.vet_name.clone();
            let vet_email = body.vet_email.clone();
            let vet_phone_number = body.vet_phone_number.clone();
            let vet_license_number = body.vet_license_number.clone();

            let query_result = vet_queries
                .update_vet(
                    vet_id.clone(),
                    vet_name,
                    vet_email,
                    vet_phone_number,
                    vet_license_number,
                )
                .await;

            if let Err(err) = query_result {
                return handle_duplicate_entry_error(err, "Vet");
            }

            match vet_queries.select_vet(vet_id.clone()).await {
                Ok(vet) => {
                    let response = json!({
                        "status": "success",
                        "message": "Vet updated successfully",
                        "data": json!({ "vet": filter_db_record(&vet) })
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

fn validate_vet_fields(body: &UpdateVet) -> Result<(), String> {
    validate_field(&body.vet_name, "Vet name cannot be empty")?;
    validate_field(&body.vet_email, "Vet email cannot be empty")?;
    validate_field(&body.vet_phone_number, "Vet phone number cannot be empty")?;
    validate_field(
        &body.vet_license_number,
        "Vet license number cannot be empty",
    )?;
    Ok(())
}
