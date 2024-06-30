use std::sync::Arc;

use crate::schemas::service_instance_schema::{
    Grooming, InsertPreventiveCare, PreventiveCare, ServiceInstance, Surgery,
};


pub struct ServiceInstanceQueries {
    db: Arc<sqlx::MySqlPool>,
    pub create_service_instance_type: &'static str,
    pub create_service_instance: &'static str,
    pub create_grooming: &'static str,
    pub create_preventive_care: &'static str,
    pub create_surgery: &'static str,
}

impl ServiceInstanceQueries {
    pub fn new(db: Arc<sqlx::MySqlPool>) -> Self {
        Self {
            db,
            create_service_instance_type: r#"
                INSERT INTO service_type (
                    service_type,
                    service_instance_id
                )
            "#,
            create_service_instance: r#"
            INSERT INTO service_instance (
            service_instance_id,
            service_date,
            service_type,
            service_reason,
            general_diagnosis,
            requires_followup,
            followup_date,
            pet_id
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            create_grooming: r#"
                INSERT INTO grooming (
                    grooming_type,
                    service_instance_id
                )
            "#,
            create_preventive_care: r#"
                INSERT INTO preventive_care (
                    treatment,
                    vet_id,
                    service_instance_id
                )
            "#,
            create_surgery: r#"
                INSERT INTO surgery (
                surgery_type,
                anesthesia_used,
                complications,
                outcome,
                service_instance_id,
                vet_id
                ) VALUES (?, ?, ?, ?, ?, ?)
            "#,
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
        preventive_care: Option<InsertPreventiveCare>,
        surgery: Option<Surgery>,
    ) -> Result<ServiceInstance, sqlx::Error> {
        let mut tx = self.db.begin().await?;

        let _service_instance = sqlx::query(&self.create_service_instance)
            .bind(service_instance_id.clone())
            .bind(service_date.clone())
            .bind(service_reason.clone())
            .bind(general_diagnosis.clone())
            .bind(requires_followup)
            .bind(followup_date.clone())
            .bind(pet_id.clone())
            .fetch_one(&mut *tx)
            .await?;

        let mut service_types = Vec::new();
        for service in service_type {
            sqlx::query(&self.create_service_instance_type)
                .bind(service)
                .bind(service_instance_id.clone())
                .execute(&mut *tx)
                .await?;
            service_types.push(service.to_string());
        }

        let mut groomings = Vec::new();
        if let Some(grooming_type) = grooming_type {
            for grooming in grooming_type {
                sqlx::query(&self.create_grooming)
                    .bind(grooming.clone())
                    .bind(service_instance_id.clone())
                    .execute(&mut *tx)
                    .await?;
                groomings.push(Grooming {
                    grooming_id: None,
                    grooming_type: grooming,
                    service_instance_id: service_instance_id.clone(),
                });
            }
        }

        let mut preventive_cares = Vec::new();
        if let Some(preventive_care) = preventive_care {
            for treatment in preventive_care.treatment {
                sqlx::query(&self.create_preventive_care)
                    .bind(treatment.clone())
                    .bind(preventive_care.vet_id.clone())
                    .bind(service_instance_id.clone())
                    .execute(&mut *tx)
                    .await?;
                preventive_cares.push(PreventiveCare {
                    preventive_care_id: None,
                    treatment,
                    service_instance_id: service_instance_id.clone(),
                    vet_id: preventive_care.vet_id.clone(),
                });
            }
        }

        let mut surgeries = Vec::new();
        if let Some(surgery) = surgery {
            sqlx::query(&self.create_surgery)
                .bind(surgery.surgery_type.clone())
                .bind(surgery.anesthesia_used.clone())
                .bind(surgery.complications.clone())
                .bind(surgery.outcome.clone())
                .bind(service_instance_id.clone())
                .bind(surgery.vet_id.clone())
                .execute(&mut *tx)
                .await?;
            surgeries.push(surgery);
        }

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
}
