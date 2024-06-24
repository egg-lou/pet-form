use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PetModel {
    pub pet_id: String,
    pub pet_name: String,
    pub pet_birth_date: NaiveDate,
    pub pet_type: String,
    pub pet_breed: String,
    pub pet_weight: Decimal,
    pub pet_color: String,
    pub owner_id: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PetModelResponse {
    pub pet_id: String,
    pub pet_name: String,
    pub pet_birth_date: String,
    pub pet_type: String,
    pub pet_breed: String,
    pub pet_weight: Decimal,
    pub pet_color: String,
    pub owner_id: String,
}
