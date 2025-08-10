use actix_web::web;
use crate::health_handler::health_check_handler;

pub
fn health_check_route(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/health",
        web::get()
            .to(health_check_handler)
    );
}