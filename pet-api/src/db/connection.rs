use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use crate::config::get_db_config;

pub async fn connect() -> Result<MySqlPool, sqlx::Error> {
    let (db_host, db_port, db_name, db_user, db_password) = match get_db_config() {
        Ok(config) => config,
        Err(e) => panic!("❌ Failed to get the Database configuration: {}", e)
    };
    let connection_string = format!("mysql://{}:{}@{}:{}/{}",
                                        db_user, db_password, db_host, db_port, db_name);
    MySqlPoolOptions::new()
        .max_connections(5)
        .max_lifetime(std::time::Duration::from_secs(1800))
        .connect(&connection_string)
        .await
}