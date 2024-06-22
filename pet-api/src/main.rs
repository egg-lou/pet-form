mod config;
mod db;
mod handlers;
mod models;
mod route;
mod utils;

use std::sync::Arc;
use log::{info, error, Level};
use simple_logger::SimpleLogger;
use sqlx::MySqlPool;
use tower_http::cors::CorsLayer;
use crate::db::connection::connect;
use crate::route::create_router;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};

pub struct AppState {
    db: MySqlPool
}

#[tokio::main]
async fn main() {
    SimpleLogger::new().with_level(Level::Info.to_level_filter()).init().unwrap();
    let pool = match connect().await {
        Ok(pool) => {
            info!("✅  Connected to the Database!");
            pool
        },
        Err(e) => {
            error!("❌  Failed to connect to the Database: {}", e);
            return;
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState {db: pool.clone()})).layer(cors);

    info!("🚀 Server stared successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
