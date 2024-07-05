use std::sync::Arc;

use sqlx::Row;

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

    pub async fn select_vet(
        &self,
        vet_id: String,
    ) -> Result<crate::models::vet_model::VetModel, sqlx::Error> {
        sqlx::query_as("SELECT * FROM veterinarian WHERE vet_id = ?")
            .bind(vet_id)
            .fetch_one(&*self.db)
            .await
    }

    pub async fn select_all_vets(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<crate::models::vet_model::VetModel>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM veterinarian ORDER by vet_name LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(&*self.db)
            .await
    }

    pub async fn vet_lists(&self) -> Result<Vec<crate::models::vet_model::GetVets>, sqlx::Error> {
        sqlx::query_as("SELECT vet_id, vet_name FROM veterinarian")
            .fetch_all(&*self.db)
            .await
    }

    pub async fn delete_vet(&self, vet_id: String) -> Result<u64, sqlx::Error> {
        sqlx::query("DELETE FROM veterinarian WHERE vet_id = ?")
            .bind(vet_id)
            .execute(&*self.db)
            .await
            .map(|done| done.rows_affected())
    }

    pub async fn update_vet(
        &self,
        vet_id: String,
        vet_name: Option<String>,
        vet_email: Option<String>,
        vet_phone_number: Option<String>,
        vet_license_number: Option<String>,
    ) -> Result<u64, sqlx::Error> {
        let mut query_string = String::from("UPDATE veterinarian SET ");
        let mut params = Vec::new();

        if let Some(vet_name) = vet_name {
            query_string.push_str("vet_name = ?, ");
            params.push(vet_name);
        }

        if let Some(vet_email) = vet_email {
            query_string.push_str("vet_email = ?, ");
            params.push(vet_email);
        }

        if let Some(vet_phone_number) = vet_phone_number {
            query_string.push_str("vet_phone_number = ?, ");
            params.push(vet_phone_number);
        }

        if let Some(vet_license_number) = vet_license_number {
            query_string.push_str("vet_license_number = ?, ");
            params.push(vet_license_number);
        }

        if query_string.ends_with(", ") {
            query_string.truncate(query_string.len() - 2);
        }

        query_string.push_str(" WHERE vet_id = ?");
        params.push(vet_id);

        let mut query = sqlx::query(&query_string);

        for param in params {
            query = query.bind(param);
        }

        let result = query.execute(&*self.db).await?;

        Ok(result.rows_affected())
    }

    pub async fn count_all_vets(&self) -> Result<i64, sqlx::Error> {
        sqlx::query(r#"SELECT COUNT(*) as count FROM veterinarian"#)
            .fetch_one(&*self.db)
            .await
            .map(|row: sqlx::mysql::MySqlRow| row.get("count"))
    }
}
