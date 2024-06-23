use std::sync::Arc;
use crate::models::owner::OwnerModel;

pub struct OwnerQueries {
    db: Arc<sqlx::MySqlPool>
}

impl OwnerQueries {
    pub fn new(db: Arc<sqlx::MySqlPool>) -> Self {
        Self { db }
    }

    pub async fn insert_owner(
        &self,
        owner_id: String,
        owner_name: String,
        owner_email: String,
        owner_phone_number: String,
        owner_address: String,
    ) -> Result<u64,sqlx::Error> {
        sqlx::query(r#"INSERT INTO owner (owner_id, owner_name, owner_email, owner_phone_number, owner_address) VALUES (?, ?, ?, ?, ?)"#)
            .bind(owner_id)
            .bind(owner_name)
            .bind(owner_email)
            .bind(owner_phone_number)
            .bind(owner_address)
            .execute(&*self.db)
            .await
            .map(|done| done.rows_affected())
    }

    pub async fn select_owner(
        &self,
        owner_id: String,
    ) -> Result<OwnerModel, sqlx::Error> {
        sqlx::query_as("SELECT * FROM owner WHERE owner_id = ?")
            .bind(owner_id)
            .fetch_one(&*self.db)
            .await
    }

    pub async fn select_all_owners(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<OwnerModel>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM owner LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(&*self.db)
            .await
    }

    pub async fn delete_owner(
        &self,
        owner_id: String,
    ) -> Result<u64, sqlx::Error> {
        sqlx::query("DELETE FROM owner WHERE id = ?")
            .bind(owner_id)
            .execute(&*self.db)
            .await
            .map(|done| done.rows_affected())
    }


}

