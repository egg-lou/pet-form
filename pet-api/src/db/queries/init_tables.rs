pub async fn create_tables(pool: &sqlx::MySqlPool) -> Result<(), sqlx::Error> {
    let sql = include_str!("../../sql/create_tables.sql");
    let statements: Vec<&str> = sql.split(';').collect();

    for statement in statements {
        if !statement.trim().is_empty() {
            sqlx::query(statement).execute(pool).await?;
        }
    }

    Ok(())
}