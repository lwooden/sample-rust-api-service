use super::models::*;
use super::state::AppState;
use actix_web::{web, HttpResponse, Responder, HttpRequest};
use std::env;
use serde::Serialize;

// Health Check Handler
pub async fn health_check(app_state: web::Data<AppState>) -> HttpResponse {
    println!("Got a request!");

    // read application state
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);

    // update vist_count property
    *visit_count += 1;

    HttpResponse::Ok().json(&response)
}

pub async fn fetch_cat_facts() -> HttpResponse {


    let url = "https://cat-fact.herokuapp.com/facts/random";

    let client = reqwest::Client::new();

    let response = client.get(url).send().await.unwrap();

    match response.status() {
        reqwest::StatusCode::OK => match response.json::<CatFact>().await {
            Ok(parsed) => HttpResponse::Ok().json(parsed),
            Err(_) => HttpResponse::BadRequest().json("Something went wrong!"),
        },
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    }
}

pub async fn generate_message() -> HttpResponse {

    let environment = env::var("ENV").expect("ENV is not set in .env file");

    #[derive(Serialize)]
    struct Message {
        text: String
    }
    
    let message1 = Message {
        text: "It looks like you don't have access to the internet! But that's ok! Everything is working here!".to_string()
    };


    let message2 = Message {
        text: "You have access to the internet! Head over to /public endpoint and learn some facts about cats!".to_string()
    };
     

    let env = match env::var_os("ENV") {
        Some(e) => e.into_string().unwrap(),
        None => panic!("$ENV is not set")
    };  
        
    if env == "C2S" {
        HttpResponse::Ok().json(message1)
    } else {
        HttpResponse::Ok().json(message2)
    }

 }
