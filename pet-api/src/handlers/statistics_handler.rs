use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::db::queries::statistic_queries::StatisticQueries;
use crate::AppState;

pub async fn counter_services(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let statistic_queries = StatisticQueries::new(Arc::new(data.db.clone()));

    let services = statistic_queries.count_services_by_type().await;

    match services {
        Ok(services) => {
            let response = serde_json::json!({
                "status": "success",
                "message": "Services fetched successfully",
                "services": services
            });

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "message": format!("{:?}", e)
            })),
        )),
    }
}

pub async fn pet_type_visit_summery(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let statistic_queries = StatisticQueries::new(Arc::new(data.db.clone()));

    let pet_type_visit_summary = statistic_queries.get_pet_type_visit_summary().await;

    match pet_type_visit_summary {
        Ok(pet_type_visit_summary) => {
            let response = serde_json::json!({
                "status": "success",
                "message": "Pet type visit summary fetched successfully",
                "pet_type_visit_summary": pet_type_visit_summary
            });

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "message": format!("{:?}", e)
            })),
        )),
    }
}
