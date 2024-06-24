use crate::db::queries::pet_queries::PetQueries;
use crate::schemas::helper::FilterOptions;
use crate::schemas::pet::AddPet;
use crate::utils::model_to_response::filter_db_record;
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::sync::Arc;

pub async fn get_pets(
    State(data): State<Arc<AppState>>,
    opts: Option<Query<FilterOptions>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let pet_queries = PetQueries::new(Arc::new(data.db.clone()));

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let pets = pet_queries
        .select_all_pets(limit as i32, offset as i32)
        .await;

    match pets {
        Ok(pets) => {
            let response = json!({
                "status":"success",
                "message":"Pets fetched successfully",
                "data": json!({
                    "pets": pets.into_iter().map(|model| filter_db_record(&model)).collect::<Vec<_>>()
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
                "data": json!({
                    "pet": filter_db_record(&pet)
                })
            });
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }

    pub async fn delete_pet(
        Path(pet_id): Path<String>,
        State(data): State<Arc<AppState>>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let pet_queries = PetQueries::new(Arc::new(data.db.clone()));
        match pet_queries.delete_pet(pet_id).await {
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    let response = json!({
                        "status":"success",
                        "message":"Pet deleted successfully"
                    });

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
}
