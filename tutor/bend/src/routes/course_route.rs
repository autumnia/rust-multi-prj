use actix_web::web;

use crate::course_handler::{get_course_detail_handler, get_courses_handler, new_course_handler};

pub
fn get_courses_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(new_course_handler))
            .route("/{tutor_id}", web::get().to(get_courses_handler))
            .route("/{tutor_id}/{course_id}", web::get().to(get_course_detail_handler))
    );
}
