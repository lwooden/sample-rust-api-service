use dotenv::dotenv;
use actix_web::{web, App, HttpServer, middleware::Logger};
use std::io;
use std::sync::Mutex;



#[path = "./state.rs"]
mod state;

#[path = "./routes.rs"]
mod routes;

#[path = "./handlers.rs"]
mod handlers;

#[path = "./models.rs"]
mod models;

use state::AppState;
use routes::*;




#[actix_web::main]
async fn main() -> io::Result<()> {

    // dotenv().ok();

   env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // initialize state object and set visit count to 0
    let shared_data = web::Data::new(AppState { 
        health_check_response: "Cat service is up and running!".to_string(),
        visit_count: Mutex::new(0),
    });

    let app = move || {
        App::new()
        .wrap(Logger::default())
        .app_data(shared_data.clone())
        .configure(general_routes)
        .configure(course_routes)
    };

    HttpServer::new(app).bind("0.0.0.0:3000")?.run().await

}
