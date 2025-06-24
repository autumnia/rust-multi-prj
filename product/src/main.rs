mod home_controller;
use crate::home_controller::root;

mod product_controller;

use axum::{
    routing::get,
    routing::post,
    Router, // http::StatusCode,
            // Json
};

use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

// use tracing_subscriber::fmt::layer;


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // add cors
    let cors = CorsLayer::new().allow_origin(Any);

    // add database
    // dotenv::dotenv().ok();
    // let database_url = std::env::var("DATABASE_URL")
    //     .expect("DATABASE_URL not set");
    let database_url = "postgres://postgres:0823@localhost:5432/postgres".to_string();
    let pool = PgPool::connect(&database_url)
    .await
    .expect("Error with pool connection");

    // add postgres table
    let _result = sqlx::query(
        r#" CREATE TABLE IF NOT EXISTS products (
            id serial,
            name text,
            price integer
        );"#
    )
    .execute(&pool)
    .await;

    // router
    let app = Router::new()
        .route("/", get(root))
        .route("/api/v1/products", post(product_controller::create_product))
        .route("/api/v1/products", get(product_controller::get_products))
        .with_state(pool)
        .layer(cors);

    tracing::debug!("listening on port {}", "0.0.0.0:3000");
    println!("listening on port {}", "0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}


