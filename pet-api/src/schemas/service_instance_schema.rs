use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct ServiceInstance {
    pub service_instance_id: String,
    pub service_date: String,
    pub service_type: Vec<String>,
    pub service_reason: String,
    pub general_diagnosis: String,
    pub requires_followup: bool,
    pub followup_date: Option<String>,
    pub pet_id: String,
    pub grooming: Option<Vec<Grooming>>,
    pub preventive_care: Option<Vec<PreventiveCare>>,
    pub surgery: Option<Surgery>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddServiceInstance {
    pub service_type: Vec<String>,
    pub service_reason: String,
    pub general_diagnosis: String,
    pub requires_followup: bool,
    pub followup_date: Option<String>,
    pub grooming_type: Option<Vec<String>>,
    pub treatment: Option<Vec<String>>,
    pub surgery: Option<Surgery>,
    pub pet_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateServiceInstance {
    pub service_date: Option<String>,
    pub service_type: Option<String>,
    pub service_reason: Option<String>,
    pub general_diagnosis: Option<String>,
    pub requires_followup: Option<bool>,
    pub followup_date: Option<String>,
    pub grooming_type: Option<Vec<String>>,
    pub treatment: Option<Vec<String>>,
    pub surgery: Option<Surgery>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Grooming {
    pub grooming_id: Option<i32>,
    pub grooming_type: String,
    pub service_instance_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PreventiveCare {
    pub preventive_care_id: Option<i32>,
    pub treatment: String,
    pub vet_id: String,
    pub service_instance_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InsertPreventiveCare {
    pub treatment: Vec<String>,
    pub vet_id: String,
    pub service_instance_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Surgery {
    pub surgery_id: Option<i32>,
    pub surgery_type: String,
    pub anesthesia_used: Option<String>,
    pub veterinarian_diagnosis: Option<String>,
    pub complications: Option<String>,
    pub outcome: Option<String>,
    pub service_instance_id: String,
    pub vet_id: String,
}
