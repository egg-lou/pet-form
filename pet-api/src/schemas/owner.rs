use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AddOwner {
    pub owner_name: String,
    pub owner_email: String,
    pub owner_phone_number: String,
    pub owner_address: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateOwner{
    pub owner_name: Option<String>,
    pub owner_email: Option<String>,
    pub owner_phone_number: Option<String>,
    pub owner_address: Option<String>,
}
