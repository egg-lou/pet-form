use serde::{Deserialize, Serialize};

use crate::models::vet_model::VetModelForService;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct ServiceInstance {
    pub service_instance_id: String,
    pub service_date: String,
    pub service_type: String,
    pub service_reason: String,
    pub veterinarian_diagnosis: String,
    pub requires_followup: bool,
    pub followup_date: String,
    pub grooming: Option<GroomingModel>,
    pub preventive_care: Option<PreventiveCareModel>,
    pub surgery: Option<SurgeryModel>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct GroomingModel {
    pub grooming_id: i32,
    pub grooming_type: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct PreventiveCareModel {
    pub preventive_care_id: i32,
    pub treatment: String,
    pub vet: VetModelForService,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct SurgeryModel {
    pub surgery_id: i32,
    pub surgery_name: String,
    pub anesthesia_used: String,
    pub complications: String,
    pub outcome: String,
    pub vet: VetModelForService,
}
