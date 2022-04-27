use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        .route("/public", web::get().to(fetch_cat_facts))
        .route("/private", web::get().to(generate_message))
    );

}