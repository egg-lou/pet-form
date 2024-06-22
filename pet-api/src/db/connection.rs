use sqlx::mysql::MySqlPool;
use crate::config::get_db_config;

pub async fn connect() -> Result<MySqlPool, sqlx::Error> {
    let (db_host, db_port, db_name, db_user, db_password) = match get_db_config() {
        Ok(config) => config,
        Err(e) => panic!("❌Failed to get the Database configuration: {}", e)
    };
    let mut connection_string = format!("mysql://{}:{}@{}:{}/{}",
                                        db_user, db_password, db_host, db_port, db_name);
    MySqlPool::connect(&mut connection_string).await
}