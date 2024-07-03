use std::sync::Arc;

use sqlx::Error;

use crate::models::statistics_model::{PetVisitSummary, ServiceTypeCount};

pub struct StatisticQueries {
    db: Arc<sqlx::MySqlPool>,
}

impl StatisticQueries {
    pub fn new(db: Arc<sqlx::MySqlPool>) -> Self {
        StatisticQueries { db }
    }

    pub async fn count_services_by_type(&self) -> Result<Vec<ServiceTypeCount>, Error> {
        let records = sqlx::query_as::<_, ServiceTypeCount>(
            r#"
                SELECT service_type.service_type_name, COUNT(*) as total
                FROM service_instance
                JOIN service_type on service_instance.service_instance_id = service_type
                .service_instance_id
                GROUP BY service_type.service_type_name
            "#,
        )
        .fetch_all(&*self.db)
        .await?;

        Ok(records)
    }

    pub async fn get_pet_type_visit_summary(&self) -> Result<Vec<PetVisitSummary>, Error> {
        sqlx::query_as::<_, PetVisitSummary>(
            r#"
                SELECT pet.pet_type, COUNT(service_instance.service_instance_id) AS total_visits
                FROM pet
                JOIN service_instance ON pet.pet_id = service_instance.pet_id
                WHERE pet.pet_type IN ('Dog', 'Cat')
                GROUP BY pet.pet_type
                HAVING COUNT(service_instance.service_instance_id) > 1
                ORDER BY total_visits DESC
            "#,
        )
        .fetch_all(&*self.db)
        .await
    }
}
