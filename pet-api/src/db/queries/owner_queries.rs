use std::sync::Arc;

use sqlx::Row;

use crate::models::{
    owner_model::{OwnerModel, OwnerWithPets},
    pet_model::PetModel,
};

pub struct OwnerQueries {
    db: Arc<sqlx::MySqlPool>,
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
    ) -> Result<u64, sqlx::Error> {
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

    pub async fn select_owner(&self, owner_id: String) -> Result<OwnerModel, sqlx::Error> {
        sqlx::query_as("SELECT * FROM owner WHERE owner_id = ?")
            .bind(owner_id)
            .fetch_one(&*self.db)
            .await
    }

    pub async fn select_all_owners(
        &self,
        limit: i32,
        offset: i32,
        search: Option<String>,
    ) -> Result<Vec<OwnerModel>, sqlx::Error> {
        let mut query = String::from("SELECT * FROM owner ");

        if let Some(search_term) = search {
            query.push_str("WHERE owner_name LIKE ? ");
            query.push_str("ORDER BY owner_name ");
            query.push_str("LIMIT ? OFFSET ?");

            sqlx::query_as::<_, OwnerModel>(&query)
                .bind(format!("%{}%", search_term))
                .bind(limit)
                .bind(offset)
                .fetch_all(&*self.db)
                .await
        } else {
            query.push_str("ORDER BY owner_name ");
            query.push_str("LIMIT ? OFFSET ?");

            sqlx::query_as::<_, OwnerModel>(&query)
                .bind(limit)
                .bind(offset)
                .fetch_all(&*self.db)
                .await
        }
    }

    pub async fn delete_owner(&self, owner_id: String) -> Result<u64, sqlx::Error> {
        sqlx::query("DELETE FROM owner WHERE owner_id = ?")
            .bind(owner_id)
            .execute(&*self.db)
            .await
            .map(|done| done.rows_affected())
    }
    pub async fn update_owner(
        &self,
        owner_id: String,
        owner_name: Option<String>,
        owner_email: Option<String>,
        owner_phone_number: Option<String>,
        owner_address: Option<String>,
    ) -> Result<u64, sqlx::Error> {
        let mut query_string = String::from("UPDATE owner SET ");
        let mut params = Vec::new();

        if let Some(name) = owner_name {
            query_string.push_str("owner_name = ?, ");
            params.push(name);
        }

        if let Some(email) = owner_email {
            query_string.push_str("owner_email = ?, ");
            params.push(email);
        }

        if let Some(phone_number) = owner_phone_number {
            query_string.push_str("owner_phone_number = ?, ");
            params.push(phone_number);
        }

        if let Some(address) = owner_address {
            query_string.push_str("owner_address = ?, ");
            params.push(address);
        }

        if query_string.ends_with(", ") {
            query_string.truncate(query_string.len() - 2);
        }

        query_string.push_str(" WHERE owner_id = ?");
        params.push(owner_id);

        let mut query = sqlx::query(&query_string);

        for param in params {
            query = query.bind(param);
        }

        let result = query.execute(&*self.db).await?;

        Ok(result.rows_affected())
    }

    pub async fn get_owner_and_pets(&self, owner_id: String) -> Result<OwnerWithPets, sqlx::Error> {
        let owner = match self.select_owner(owner_id.clone()).await {
            Ok(owner) => owner,
            Err(_) => return Err(sqlx::Error::RowNotFound),
        };

        let pets =
            sqlx::query_as::<_, PetModel>("SELECT * FROM PET WHERE owner_id = ? ORDER BY pet_name")
                .bind(owner_id)
                .fetch_all(&*self.db)
                .await?;

        Ok(OwnerWithPets { owner, pets })
    }

    pub async fn count_all_owners(&self, search: Option<String>) -> Result<i64, sqlx::Error> {
        let mut query = String::from("SELECT COUNT(*) as count FROM owner ");

        if let Some(search_term) = search {
            query.push_str(&format!("WHERE owner_name LIKE '%{}%' ", search_term));
        }

        let query = sqlx::query(&query);

        query
            .fetch_one(&*self.db)
            .await
            .map(|row: sqlx::mysql::MySqlRow| row.get("count"))
    }
}
