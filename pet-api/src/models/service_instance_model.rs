use serde::{Deserialize, Serialize};

use crate::models::vet_model::VetModelForService;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct ServiceInstanceModel {
    pub service_instance_id: String,
    pub service_date: chrono::NaiveDate,
    pub service_type: Vec<String>,
    pub service_reason: String,
    pub general_diagnosis: String,
    pub requires_followup: bool,
    pub pet_id: String,
    pub followup_date: Option<chrono::NaiveDate>,
    pub grooming: Option<Vec<GroomingModel>>,
    pub preventive_care: Option<Vec<PreventiveCareModel>>,
    pub surgery: Option<Vec<SurgeryModel>>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct AllServiceInstanceModel {
    pub service_instance_id: String,
    pub service_date: chrono::NaiveDate,
    pub service_type: Vec<String>,
    pub pet: SimplePetModel,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SimplePetModel {
    pub pet_id: String,
    pub pet_name: String,
    pub pet_type: String,
    pub pet_breed: String,
    pub owner_name: String,
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
    pub veterinarian_diagnosis: Option<String>,
    pub anesthesia_used: Option<String>,
    pub complications: Option<String>,
    pub outcome: Option<String>,
    pub vet: VetModelForService,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct GetServicesHistoryModel {
    pub service_instance_id: String,
    pub service_date: chrono::NaiveDate,
    pub service_type: Vec<String>,
    pub service_reason: String,
    pub general_diagnosis: String,
    pub requires_followup: bool,
    pub followup_date: Option<chrono::NaiveDate>,
}
