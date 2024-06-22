mod config;
mod db;


#[tokio::main]
async fn main() {
    match db::connection::connect().await {
        Ok(_pool) => println!("✅ Connected to the Database!"),
        Err(e) => println!("❌ Failed to connect to the Database: {}", e)
    }
}
