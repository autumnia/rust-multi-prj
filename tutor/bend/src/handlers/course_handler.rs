use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::Error;
use crate::app_error::AppError;
use crate::appstate::AppState;
use crate::course_route::get_courses_route;
use crate::db::{delete_course_db, get_course_detail_db, get_courses_db, new_course_db};
use crate::models::Course;


pub async
fn new_course_handler(app_state: web::Data<AppState>, new_course: web::Json<Course>) -> Result<HttpResponse, AppError> {
    // println!("create course");
    new_course_db(&app_state.db, new_course.into_inner())
        .await
        .map(| course| HttpResponse::Ok().json(course))
}

pub async
fn get_courses_handler(app_state: web::Data<AppState>, path: web::Path<i32> ) -> Result<HttpResponse, AppError> {
    // println!("get all courses");

    let tutor_id = path.into_inner();
    get_courses_db(&app_state.db, tutor_id)
        .await
        .map(| courses| HttpResponse::Ok().json(courses))
}

pub async
fn get_course_detail_handler(app_state: web::Data<AppState>, path: web::Path<(i32, i32)> ) -> Result<HttpResponse, AppError> {
    let (tutor_id, course_id) = path.into_inner();

    get_course_detail_db( &app_state.db, tutor_id, course_id )
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async
fn delete_course(app_state: web::Data<AppState>, path: web::Path<(i32, i32)>, ) -> Result<HttpResponse, AppError> {
    let (tutor_id, course_id) = path.into_inner();
    delete_course_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;
    use actix_web::web::Data;
    use dotenv::dotenv;
    use sqlx::{PgPool, Pool, Postgres};

    async
    fn get_pool() -> Pool<Postgres> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool = PgPool::connect(&database_url).await.unwrap();
        pool
    }

    fn get_app_state(pool: Pool<Postgres>) -> Data<AppState> {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        app_state
    }

    #[actix_rt::test]
    async
    fn new_course_test() {
        let course = web::Json(Course {
            tutor_id: 1,
            course_name: "Hello, this is a test course".into(),
            course_id: None,
            posted_time: None,
        });

        dotenv().ok();
        let pool = get_pool().await;

        let app_state = get_app_state(pool);

        let res = new_course(app_state, course).await;

        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async
    fn get_all_courses_test() {
        dotenv().ok();
        let pool = get_pool().await;

        let app_state = get_app_state(pool);

        let tutor_id: web::Path<(i32,)> = web::Path::from((1,));

        let res = get_courses(app_state, tutor_id).await;

        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async
    fn get_one_course_test() {
        dotenv().ok();

        let pool = get_pool().await;

        let app_state = get_app_state(pool);

        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));

        let res = get_course_detail(app_state, params).await;

        assert_eq!(res.status(), StatusCode::OK);
    }
}