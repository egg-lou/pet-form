use dotenv::dotenv;
use std::env;
use tower_http::cors::{Cors, CorsLayer};

pub fn get_db_config() -> Result<(String, u16, String, String, String), env::VarError> {
    dotenv().ok();

    let db_host = env::var("DB_HOST")?;
    let db_port = env::var("DB_PORT")?.parse::<u16>().map_err(|_| env::VarError::NotPresent)?;
    let db_name = env::var("DB_NAME")?;
    let db_user = env::var("DB_USER")?;
    let db_password = env::var("DB_PASSWORD")?;

    Ok((db_host, db_port, db_name, db_user, db_password))
}