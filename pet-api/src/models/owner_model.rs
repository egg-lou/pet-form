use serde::{Deserialize, Serialize};

use crate::models::pet_model::PetModel;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OwnerModel {
    pub owner_id: String,
    pub owner_name: String,
    pub owner_email: String,
    pub owner_phone_number: String,
    pub owner_address: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OwnerModelResponse {
    pub owner_id: String,
    pub owner_name: String,
    pub owner_email: String,
    pub owner_phone_number: String,
    pub owner_address: String,
}

pub struct OwnerWithPets {
    pub owner: OwnerModel,
    pub pets: Vec<PetModel>,
}
