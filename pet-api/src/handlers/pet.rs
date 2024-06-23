use std::sync::Arc;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use crate::AppState;
use crate::schemas::helper::FilterOptions;
use crate::db::queries::pet_queries::PetQueries;

pub async fn get_pets(
    opts:Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) -1) * limit;

    let pet_queries = PetQueries::new(Arc::new(data.db.clone()));

    let pets = pet_queries.select_all_pets(limit as i32, offset as i32).await;

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
        },
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        ))
    }
}