use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        // .route("/public", web::get().to(fetch_cat_facts))
        .route("/pokemon/{id}", web::get().to(get_pokemon))
        .route("/echo", web::get().to(get_echo))
        .route("/env", web::get().to(check_environment))
        .route("kubernetes",web::get().to(get_podInfo))
        .route("/sum",web::post().to(get_sum))
    );

}