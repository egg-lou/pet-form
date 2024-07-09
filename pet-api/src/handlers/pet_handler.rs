use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::db::queries::pet_queries::PetQueries;
use crate::schemas::helper_schema::FilterOptions;
use crate::schemas::pet_schema::{AddPet, UpdatePet};
use crate::utils::handle_duplicate_error::handle_duplicate_entry_error;
use crate::utils::{model_to_response::filter_db_record, validator::validate_field};
use crate::AppState;

pub async fn get_pets(
    State(data): State<Arc<AppState>>,
    opts: Option<Query<FilterOptions>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let pet_queries = PetQueries::new(Arc::new(data.db.clone()));

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;
    let search = opts.search.clone();
    let total_pets = pet_queries
        .count_all_pets(search.clone())
        .await
        .unwrap_or_default();
    let total_pages = (total_pets as f64 / limit as f64).ceil() as i32;

    let pets = pet_queries
        .select_all_pets(limit as i32, offset as i32, search)
        .await;

    match pets {
        Ok(pets) => {
            let response = json!({
                "status":"success",
                "message":"Pets fetched successfully",
                "pets": pets.into_iter().map(|model| filter_db_record(&model)).collect::<Vec<_>>(),
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

pub async fn add_pet(
    State(data): State<Arc<AppState>>,
    Json(body): Json<AddPet>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let pet_id = uuid::Uuid::new_v4().to_string();
    let pet_queries = PetQueries::new(Arc::new(data.db.clone()));
    let _pet = pet_queries
        .insert_pet(
            pet_id.clone(),
            body.pet_name.to_string(),
            body.pet_birth_date,
            body.pet_type.to_string(),
            body.pet_breed.to_string(),
            body.pet_weight,
            body.pet_color.to_string(),
            body.owner_id.to_string(),
        )
        .await;

    match pet_queries.select_pet(pet_id.clone()).await {
        Ok(pet) => {
            let response = json!({
                "status":"success",
                "message":"Pet added successfully",
                "pet": filter_db_record(&pet)
            });
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

pub async fn delete_pet(
    Path(pet_id): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let pet_queries = PetQueries::new(Arc::new(data.db.clone()));
    match pet_queries.delete_pet(pet_id).await {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                let response = json!({ "status":"success", "message":"Pet deleted successfully" });
                Ok((StatusCode::OK, Json(response)))
            } else {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(json!({"status": "error", "message": "Pet not found"})),
                ))
            }
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

pub async fn update_pet(
    Path(pet_id): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdatePet>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if let Err(message) = validate_pet_fields(&body) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"status": "error", "message": message})),
        ));
    }

    let pet_queries = PetQueries::new(Arc::new(data.db.clone()));
    let pet = pet_queries.select_pet(pet_id.clone()).await;

    match pet {
        Ok(_) => {
            let pet_name = body.pet_name.clone();
            let pet_birth_date = body.pet_birth_date.clone();
            let pet_type = body.pet_type.clone();
            let pet_breed = body.pet_breed.clone();
            let pet_weight = body.pet_weight.clone();
            let pet_color = body.pet_color.clone();
            let owner_id = body.owner_id.clone();

            let query_result = pet_queries
                .update_pet(
                    pet_id.clone(),
                    pet_name,
                    pet_birth_date,
                    pet_type,
                    pet_breed,
                    pet_weight,
                    pet_color,
                    owner_id,
                )
                .await;

            if let Err(err) = query_result {
                return handle_duplicate_entry_error(err, "Pet");
            }

            match pet_queries.select_pet(pet_id.clone()).await {
                Ok(pet) => {
                    let response = json!({
                        "status": "success",
                        "message": "Pet updated successfully",
                            "pet": filter_db_record(&pet)
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

pub async fn get_pet(
    Path(pet_id): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let pet_queries = PetQueries::new(Arc::new(data.db.clone()));
    match pet_queries.select_pet_details(pet_id).await {
        Ok(pet) => {
            let response = json!({
                "status":"success",
                "message":"Pet fetched successfully",
                "pet": filter_db_record(&pet)
            });

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

fn validate_pet_fields(body: &UpdatePet) -> Result<(), String> {
    validate_field(&body.pet_name, "Pet name cannot be empty")?;
    validate_field(&body.pet_type, "Pet type cannot be empty")?;
    validate_field(&body.pet_breed, "Pet breed cannot be empty")?;
    validate_field(&body.pet_color, "Pet color cannot be empty")?;
    validate_field(&body.owner_id, "Owner ID cannot be empty")?;

    if let Some(field) = &body.pet_weight {
        if *field < 0.0 {
            return Err("Pet weight cannot be negative".to_string());
        }
    }

    Ok(())
}
