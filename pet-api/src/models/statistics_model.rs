use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct ServiceTypeCount {
    service_type_name: String,
    total: i64,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct PetVisitSummary {
    pet_type: String,
    total_visits: i64,
}
