use core::option::Option;
use std::sync::Arc;

use sqlx::Row;

use crate::models::service_instance_model::{
    GetServicesHistoryModel, GroomingModel, PreventiveCareModel, ServiceInstanceModel, SurgeryModel,
};
use crate::models::vet_model::VetModelForService;
use crate::schemas::service_instance_schema::{
    AddPreventiveCare, AddSurgery, Grooming, PreventiveCare, ServiceInstance, Surgery,
};

pub struct ServiceInstanceQueries {
    db: Arc<sqlx::MySqlPool>,
    pub create_service_instance_type: &'static str,
    pub create_service_instance: &'static str,
    pub create_grooming: &'static str,
    pub create_preventive_care: &'static str,
    pub create_surgery: &'static str,
    pub get_service_instance_of_pet: &'static str,
    pub get_specific_service_instance: &'static str,
    pub get_grooming_of_service_instance: &'static str,
    pub get_preventive_care_of_service_instance: &'static str,
    pub get_surgery_of_service_instance: &'static str,
    pub get_service_instance_type: &'static str,
}

impl ServiceInstanceQueries {
    pub fn new(db: Arc<sqlx::MySqlPool>) -> Self {
        Self {
            db,
            create_service_instance_type: r#"INSERT INTO service_type ( service_type_name, service_instance_id) VALUES (?, ?)"#,
            create_service_instance: r#"INSERT INTO service_instance (service_instance_id, service_date, service_reason,
            general_diagnosis, requires_followup, followup_date, pet_id) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            create_grooming: r#"INSERT INTO grooming (grooming_type, service_instance_id) VALUES (?, ?)"#,
            create_preventive_care: r#"INSERT INTO preventive_care (treatment, vet_id,
            service_instance_id) VALUES (?, ?, ?)"#,
            create_surgery: r#"INSERT INTO surgery (surgery_name, anesthesia_used, complications,
             outcome, service_instance_id, vet_id) VALUES (?, ?, ?, ?, ?, ?)"#,
            get_service_instance_of_pet: r#"SELECT * FROM service_instance WHERE pet_id = ?"#,
            get_specific_service_instance: r#"SELECT * FROM service_instance WHERE service_instance_id = ?"#,
            get_grooming_of_service_instance: r#"SELECT * FROM grooming WHERE service_instance_id = ?"#,
            get_preventive_care_of_service_instance: r#"SELECT * FROM preventive_care WHERE service_instance_id = ?"#,
            get_surgery_of_service_instance: r#"SELECT * FROM surgery WHERE service_instance_id = ?"#,
            get_service_instance_type: r#"SELECT service_type_name FROM service_type WHERE service_instance_id = ?"#,
        }
    }

    pub async fn insert_service_instance(
        &self,
        service_instance_id: String,
        service_date: String,
        service_type: Vec<String>,
        service_reason: String,
        general_diagnosis: String,
        requires_followup: bool,
        followup_date: Option<String>,
        pet_id: String,
        grooming_type: Option<Vec<String>>,
        preventive_care: Option<AddPreventiveCare>,
        surgery: Option<AddSurgery>,
    ) -> Result<ServiceInstance, sqlx::Error> {
        let mut tx = self.db.begin().await?;

        let _service = sqlx::query(&self.create_service_instance)
            .bind(service_instance_id.clone())
            .bind(service_date.clone())
            .bind(service_reason.clone())
            .bind(general_diagnosis.clone())
            .bind(requires_followup)
            .bind(followup_date.clone())
            .bind(pet_id.clone())
            .execute(&mut *tx)
            .await?;

        let mut service_types = Vec::new();
        for service in &service_type {
            sqlx::query(&self.create_service_instance_type)
                .bind(service.clone())
                .bind(service_instance_id.clone())
                .execute(&mut *tx)
                .await?;
            service_types.push(service.to_string());
        }

        let mut groomings = Vec::new();
        if let Some(grooming_type) = grooming_type {
            for grooming in grooming_type {
                let row = sqlx::query(&self.create_grooming)
                    .bind(grooming.clone())
                    .bind(service_instance_id.clone())
                    .execute(&mut *tx)
                    .await?;

                let grooming_id = row.last_insert_id() as i32;

                groomings.push(Grooming {
                    grooming_id: Some(grooming_id),
                    grooming_type: grooming,
                    service_instance_id: service_instance_id.clone(),
                });
            }
        }

        let mut preventive_cares = Vec::new();
        if let Some(preventive_care) = preventive_care {
            for treatment in preventive_care.treatment {
                let row = sqlx::query(&self.create_preventive_care)
                    .bind(treatment.clone())
                    .bind(preventive_care.vet_id.clone())
                    .bind(service_instance_id.clone())
                    .execute(&mut *tx)
                    .await?;
                let preventive_care_id = row.last_insert_id() as i32;
                preventive_cares.push(PreventiveCare {
                    preventive_care_id: Some(preventive_care_id),
                    treatment,
                    service_instance_id: service_instance_id.clone(),
                    vet_id: preventive_care.vet_id.clone(),
                });
            }
        }

        let mut surgeries = Vec::new();
        if let Some(surgery) = surgery {
            let row = sqlx::query(&self.create_surgery)
                .bind(surgery.surgery_name.clone())
                .bind(surgery.anesthesia_used.clone())
                .bind(surgery.complications.clone())
                .bind(surgery.outcome.clone())
                .bind(service_instance_id.clone())
                .bind(surgery.vet_id.clone())
                .execute(&mut *tx)
                .await?;

            let surgery_id = row.last_insert_id() as i32;
            surgeries.push(Surgery {
                surgery_id: Some(surgery_id),
                surgery_name: surgery.surgery_name.clone(),
                anesthesia_used: surgery.anesthesia_used,
                veterinarian_diagnosis: surgery.veterinarian_diagnosis,
                complications: surgery.complications,
                outcome: surgery.outcome,
                vet_id: surgery.vet_id,
                service_instance_id: service_instance_id.clone(),
            });
        }

        tx.commit().await?;

        Ok(ServiceInstance {
            service_instance_id,
            service_date,
            service_type,
            service_reason,
            general_diagnosis,
            requires_followup,
            followup_date,
            pet_id,
            grooming: if groomings.is_empty() {
                None
            } else {
                Some(groomings)
            },
            preventive_care: if preventive_cares.is_empty() {
                None
            } else {
                Some(preventive_cares)
            },
            surgery: if surgeries.is_empty() {
                None
            } else {
                Some(surgeries[0].clone())
            },
        })
    }
    pub async fn get_services_history_of_pet(
        &self,
        limit: i32,
        offset: i32,
        pet_id: String,
    ) -> Result<Vec<GetServicesHistoryModel>, sqlx::Error> {
        let rows = sqlx::query (
            r#"
        SELECT service_instance.*, service_type.service_type_name
        FROM service_instance
        LEFT JOIN service_type ON service_instance.service_instance_id = service_type.service_instance_id
        WHERE service_instance.pet_id = ?
        ORDER BY service_instance.service_instance_id
        LIMIT ? OFFSET ?
        "#,
        )
            .bind(pet_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&*self.db)
            .await?;

        let mut services = Vec::new();
        let mut current_service: Option<GetServicesHistoryModel> = None;

        for row in rows {
            let service_instance_id: String = row.get("service_instance_id");
            let service_type_name: String = row.get("service_type_name");

            match current_service.as_mut() {
                Some(service) if service.service_instance_id == service_instance_id => {
                    service.service_type.push(service_type_name);
                }
                _ => {
                    if let Some(service) = current_service.take() {
                        services.push(service);
                    }

                    current_service = Some(GetServicesHistoryModel {
                        service_instance_id: service_instance_id.clone(),
                        service_date: row.get("service_date"),
                        service_type: vec![service_type_name],
                        service_reason: row.get("service_reason"),
                        general_diagnosis: row.get("general_diagnosis"),
                        requires_followup: row.get("requires_followup"),
                        followup_date: row.get("followup_date"),
                    });
                }
            }
        }

        if let Some(service) = current_service {
            services.push(service);
        }

        Ok(services)
    }
    pub async fn get_specific_instance(
        &self,
        service_instance_id: String,
    ) -> Result<ServiceInstanceModel, sqlx::Error> {
        let row = sqlx::query(&self.get_specific_service_instance)
            .bind(service_instance_id.clone())
            .fetch_one(&*self.db)
            .await?;

        let service_instance = ServiceInstanceModel {
            service_instance_id: row.get("service_instance_id"),
            service_date: row.get("service_date"),
            service_type: Vec::new(),
            service_reason: row.get("service_reason"),
            general_diagnosis: row.get("general_diagnosis"),
            requires_followup: row.get("requires_followup"),
            followup_date: row.get("followup_date"),
            pet_id: row.get("pet_id"),
            grooming: None,
            preventive_care: None,
            surgery: None,
        };

        let grooming_rows = sqlx::query(&self.get_grooming_of_service_instance)
            .bind(&service_instance_id)
            .fetch_all(&*self.db)
            .await?;

        let mut groomings = Vec::new();
        for row in grooming_rows {
            groomings.push(GroomingModel {
                grooming_id: row.get("grooming_id"),
                grooming_type: row.get("grooming_type"),
            });
        }

        let preventive_care_rows = sqlx::query(&self.get_preventive_care_of_service_instance)
            .bind(&service_instance_id)
            .fetch_all(&*self.db)
            .await?;

        let mut preventive_cares = Vec::new();
        for row in preventive_care_rows {
            let vet_id: String = row.get("vet_id");
            let vet_row = sqlx::query("SELECT * FROM veterinarian WHERE vet_id = ?")
                .bind(&vet_id)
                .fetch_one(&*self.db)
                .await?;
            let vet = VetModelForService {
                vet_name: vet_row.get("vet_name"),
                vet_email: vet_row.get("vet_email"),
                vet_phone_number: vet_row.get("vet_phone_number"),
                vet_license_number: vet_row.get("vet_license_number"),
            };
            preventive_cares.push(PreventiveCareModel {
                preventive_care_id: row.get("preventive_care_id"),
                treatment: row.get("treatment"),
                vet,
            });
        }

        let surgery_rows = sqlx::query(&self.get_surgery_of_service_instance)
            .bind(&service_instance_id)
            .fetch_all(&*self.db)
            .await?;

        let mut surgeries = Vec::new();
        for row in surgery_rows {
            let vet_id: String = row.get("vet_id");
            let vet_row = sqlx::query("SELECT * FROM veterinarian WHERE vet_id = ?")
                .bind(&vet_id)
                .fetch_one(&*self.db)
                .await?;
            let vet = VetModelForService {
                vet_name: vet_row.get("vet_name"),
                vet_email: vet_row.get("vet_email"),
                vet_phone_number: vet_row.get("vet_phone_number"),
                vet_license_number: vet_row.get("vet_license_number"),
            };
            surgeries.push(SurgeryModel {
                surgery_id: row.get("surgery_id"),
                surgery_name: row.get("surgery_name"),
                anesthesia_used: row.get("anesthesia_used"),
                complications: row.get("complications"),
                outcome: row.get("outcome"),
                vet,
            });
        }

        let service_instance = ServiceInstanceModel {
            grooming: if groomings.is_empty() {
                None
            } else {
                Some(groomings)
            },
            preventive_care: if preventive_cares.is_empty() {
                None
            } else {
                Some(preventive_cares)
            },
            surgery: surgeries.into_iter().next(),
            ..service_instance
        };

        Ok(service_instance)
    }
}
