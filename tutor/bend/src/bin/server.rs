#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{env, io};
use std::sync::Mutex;
use actix_web::{App, HttpServer, Responder, web};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;

#[path = "../routes/course_route.rs"]
mod course_route;

#[path = "../routes/health_route.rs"]
mod health_route;

#[path = "../handlers/course_handler.rs"]
mod course_handler;
use crate::course_route::get_courses_route;

#[path = "../handlers/health_handler.rs"]
mod health_handler;
use crate::health_route::health_check_route;

#[path = "../states/appstate.rs"]
mod appstate;
use crate::appstate::AppState;

#[path = "../db/course_db.rs"]
mod db;

#[path = "../models/course.rs"]
mod models;

#[path = "../exceptions/app_error.rs"]
mod app_error;

#[path = "../exceptions/error_response.rs"]
mod error_response;

#[actix_rt::main]
async
fn main() -> io::Result<()>{
    // 환경 변수 로딩
    dotenv().ok();
    // check_env();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in .env file");

    // 데이터베이스 풀 생성
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await.unwrap();

    // 데이터베이스 연결 테스트
    match pool.acquire().await {
        Ok(_) => println!("Successfully connected to the database."),
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            return Err(io::Error::new(io::ErrorKind::Other, err));
        }
    }

    let shared_data = web::Data::new(AppState {
        health_check_response: "I am good. You have already asked me".to_string(),
        visit_count: Mutex::new(0),
        db: pool,
    });

    let app = move || {
         App::new()
             .app_data(shared_data.clone())
             .configure(health_check_route)
             .configure(get_courses_route)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?
        .run().await
}

fn check_env() {
    for (key, value) in std::env::vars() {
        if key == "DATABASE_URL" {
            println!("{}: {}", key, value);
        }
    }
}
