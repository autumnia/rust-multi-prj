#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::io;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    Router,
    routing::{get, post},
};
use http::{
    header,
    Method
};
use sqlx::PgPool;
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile}
};


use crate::home_controller::root;

mod home_controller;
mod product_controller;

// use tracing_subscriber::fmt::layer;

#[tokio::main]
async fn main() {
    // initialize tracing
    init_tracing_subscriber().await;

    // add cors
    let cors = CorsLayer::new().allow_origin(Any);


    // add database
    dotenv::dotenv().ok();
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
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
        .route("/home", get(root))
        .route("/api/v1/products",
               get(product_controller::get_products)
               .post(product_controller::create_product)
        )
        .route("/api/v1/products/:id",
               get(product_controller::get_one_product)
               .delete(product_controller::delete_one_product)
               .put(product_controller::update_one_product)
        )
        .with_state(pool)
        .layer(cors) ;
        // .nest_service("/", serve_dir.clone())
        // .fallback_service(serve_dir.clone());

    tracing::debug!("debut printing: listening on port {}", "0.0.0.0:3000");
    println!(" Listening on port {}" , "0.0.0.0:3000" );
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn init_tracing_subscriber() {
    tracing_subscriber::fmt::init();
}


//  add handle error fn
async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}


