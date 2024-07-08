use std::sync::Arc;

use sqlx::Row;

use crate::models::pet_model::{PetModel, PetModelResponse};


pub struct PetQueries {
    db: Arc<sqlx::MySqlPool>,
}

impl PetQueries {
    pub fn new(db: Arc<sqlx::MySqlPool>) -> Self {
        Self { db }
    }

    pub async fn insert_pet(
        &self,
        pet_id: String,
        pet_name: String,
        pet_birth_date: String,
        pet_type: String,
        pet_breed: String,
        pet_weight: f32,
        pet_color: String,
        owner_id: String,
    ) -> Result<u64, sqlx::Error> {
        sqlx::query(r#"INSERT INTO pet (pet_id, pet_name, pet_birth_date, pet_type, pet_breed, pet_weight, pet_color, owner_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(pet_id)
            .bind(pet_name)
            .bind(pet_birth_date)
            .bind(pet_type)
            .bind(pet_breed)
            .bind(pet_weight)
            .bind(pet_color)
            .bind(owner_id)
            .execute(&*self.db)
            .await
            .map(|done| done.rows_affected())
    }

    pub async fn select_pet(&self, pet_id: String) -> Result<PetModel, sqlx::Error> {
        sqlx::query_as("SELECT * FROM pet WHERE pet_id = ?")
            .bind(pet_id)
            .fetch_one(&*self.db)
            .await
    }

    pub async fn select_all_pets(
        &self,
        limit: i32,
        offset: i32,
        search: Option<String>,
    ) -> Result<Vec<PetModelResponse>, sqlx::Error> {
        let mut query = String::from(
            r#"
    SELECT pet.*, owner.owner_name, owner.owner_email
    FROM pet
    INNER JOIN owner ON pet.owner_id = owner.owner_id
    "#,
        );

        if let Some(search_term) = search {
            query.push_str("WHERE pet_name LIKE ? ");
            query.push_str("ORDER BY pet_type ");
            query.push_str("LIMIT ? OFFSET ?");

            sqlx::query_as::<_, PetModelResponse>(&query)
                .bind(format!("%{}%", search_term))
                .bind(limit)
                .bind(offset)
                .fetch_all(&*self.db)
                .await
        } else {
            query.push_str("ORDER BY pet_type ");
            query.push_str("LIMIT ? OFFSET ?");

            sqlx::query_as::<_, PetModelResponse>(&query)
                .bind(limit)
                .bind(offset)
                .fetch_all(&*self.db)
                .await
        }
    }

    pub async fn delete_pet(&self, pet_id: String) -> Result<u64, sqlx::Error> {
        sqlx::query("DELETE FROM pet WHERE pet_id = ?")
            .bind(pet_id)
            .execute(&*self.db)
            .await
            .map(|done| done.rows_affected())
    }

    pub async fn update_pet(
        &self,
        pet_id: String,
        pet_name: Option<String>,
        pet_birth_date: Option<String>,
        pet_type: Option<String>,
        pet_breed: Option<String>,
        pet_weight: Option<f32>,
        pet_color: Option<String>,
        owner_id: Option<String>,
    ) -> Result<u64, sqlx::Error> {
        let mut query_string = String::from("UPDATE pet SET ");
        let mut params = Vec::new();

        if let Some(name) = pet_name {
            query_string.push_str("pet_name = ?, ");
            params.push(name);
        }

        if let Some(birth_date) = pet_birth_date {
            query_string.push_str("pet_birth_date = ?, ");
            params.push(birth_date);
        }

        if let Some(pet_type) = pet_type {
            query_string.push_str("pet_type = ?, ");
            params.push(pet_type);
        }

        if let Some(breed) = pet_breed {
            query_string.push_str("pet_breed = ?, ");
            params.push(breed);
        }

        if let Some(weight) = pet_weight {
            query_string.push_str("pet_weight = ?, ");
            params.push(weight.to_string());
        }

        if let Some(color) = pet_color {
            query_string.push_str("pet_color = ?, ");
            params.push(color);
        }

        if let Some(owner_id) = owner_id {
            query_string.push_str("owner_id = ?, ");
            params.push(owner_id);
        }

        if query_string.ends_with(", ") {
            query_string.truncate(query_string.len() - 2);
        }

        query_string.push_str(" WHERE pet_id = ?");
        params.push(pet_id);

        let mut query = sqlx::query(&query_string);

        for param in params {
            query = query.bind(param);
        }

        let result = query.execute(&*self.db).await?;

        Ok(result.rows_affected())
    }

    pub async fn count_all_pets(&self, search: Option<String>) -> Result<i64, sqlx::Error> {
        let mut query = String::from("SELECT COUNT(*) as count FROM pet ");

        if let Some(search_term) = search {
            query.push_str(&format!("WHERE pet_name LIKE '%{}%' ", search_term));
        }

        let query = sqlx::query(&query);

        query
            .fetch_one(&*self.db)
            .await
            .map(|row: sqlx::mysql::MySqlRow| row.get("count"))
    }

    pub async fn select_pet_details(
        &self,
        pet_id: String,
    ) -> Result<PetModelResponse, sqlx::Error> {
        let query = r#"
        SELECT pet.*, owner.*
        FROM pet
        INNER JOIN owner ON pet.owner_id = owner.owner_id
        WHERE pet.pet_id = ?
        "#;

        sqlx::query_as::<_, PetModelResponse>(query)
            .bind(pet_id)
            .fetch_one(&*self.db)
            .await
    }
}
