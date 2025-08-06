use std::env;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use startup::run;
use tokio::net::TcpListener;

pub mod error;
pub mod models;
pub mod routes;
pub mod startup;
pub mod helper;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DB_URL_NOT_SET");
    let db_connect = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect db");

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    run(listener, db_connect).await
}
