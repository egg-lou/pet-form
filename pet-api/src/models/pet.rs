use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PetModel {
    pub pet_id: String,
    pub pet_name: String,
    pub pet_birth_date: String,
    pub pet_type: String,
    pub pet_breed: String,
    pub pet_weight: f32,
    pub pet_color: String,
    pub pet_owner_id: String
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PetModelResponse {
    pub pet_id: String,
    pub pet_name: String,
    pub pet_birth_date: String,
    pub pet_type: String,
    pub pet_breed: String,
    pub pet_weight: f32,
    pub pet_color: String,
    pub pet_owner_id: String
}