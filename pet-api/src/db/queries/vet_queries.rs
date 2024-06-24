use std::sync::Arc;


pub struct VetQueries {
    db: Arc<sqlx::MySqlPool>,
}

impl VetQueries {
    pub fn new(db: Arc<sqlx::MySqlPool>) -> Self {
        VetQueries { db }
    }

    pub async fn insert_vet(
        &self,
        vet_id: String,
        vet_name: String,
        vet_email: String,
        vet_phone_number: String,
        vet_license_number: String,
    ) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"INSERT INTO veterinarian (vet_id, vet_name, vet_email, vet_phone_number,
        vet_license_number) VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(vet_id)
        .bind(vet_name)
        .bind(vet_email)
        .bind(vet_phone_number)
        .bind(vet_license_number)
        .execute(&*self.db)
        .await
        .map(|done| done.rows_affected())
    }
}
