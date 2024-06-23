use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AddPet {
    pub pet_name: String,
    pub pet_birth_date: String,
    pub pet_type: String,
    pub pet_breed: String,
    pub pet_weight: f32,
    pub pet_color: String,
    pub owner_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdatePet {
    pub pet_name: Option<String>,
    pub pet_birth_date: Option<String>,
    pub pet_type: Option<String>,
    pub pet_breed: Option<String>,
    pub pet_weight: Option<f32>,
    pub pet_color: Option<String>,
    pub owner_id: Option<String>,
}