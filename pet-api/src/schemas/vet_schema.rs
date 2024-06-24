use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AddVet {
    pub vet_name: String,
    pub vet_email: String,
    pub vet_phone_number: String,
    pub vet_license_number: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateVet {
    pub vet_name: Option<String>,
    pub vet_email: Option<String>,
    pub vet_phone_number: Option<String>,
    pub vet_license_number: Option<String>,
}
