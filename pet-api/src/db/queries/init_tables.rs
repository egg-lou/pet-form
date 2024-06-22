pub async fn create_tables(pool: &sqlx::MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query(include_str!("../../sql/create_tables.sql"))
        .execute(pool)
        .await?;
    Ok(())
}