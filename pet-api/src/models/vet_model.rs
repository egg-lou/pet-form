use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct VetModel {
    pub vet_id: String,
    pub vet_name: String,
    pub vet_email: String,
    pub vet_phone_number: String,
    pub vet_license_number: String,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct GetVets {
    pub vet_id: String,
    pub vet_name: String,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct VetModelResponse {
    pub vet_id: String,
    pub vet_name: String,
    pub vet_email: String,
    pub vet_phone_number: String,
    pub vet_license_number: String,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct VetModelForService {
    pub vet_name: String,
    pub vet_email: String,
    pub vet_phone_number: String,
    pub vet_license_number: String,
}
