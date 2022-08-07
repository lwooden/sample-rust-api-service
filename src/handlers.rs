use super::models::*;
use super::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use std::env;

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

pub async fn get_echo(term: web::Query<Term>) -> HttpResponse {
    HttpResponse::Ok().json(format!("You said: {}", term.term))
}

pub async fn check_environment() -> HttpResponse {
    let env = match env::var_os("ENV") {
        Some(e) => e.into_string().unwrap(),
        None => panic!("$ENV is not set"),
    };

    let valid_env = env.is_empty();

    if valid_env == false {
        HttpResponse::Ok().json(format!(
            "It looks like you are running in a {} environment!",env
        ))
    } else {
        HttpResponse::NotFound().json("Environment not set!")
    }
}

pub async fn get_sum(payload: web::Json<Sum>) -> HttpResponse {
    let sum: i32 = payload.val_1 + payload.val_2;
    HttpResponse::Ok().json(sum)
}

pub async fn get_podInfo() -> HttpResponse {
    let node = match env::var_os("NODE_NAME") {
        Some(n) => n.into_string().unwrap(),
        None => panic!("You are not in a Kubernetes Environment!"),
    };

    let pod = match env::var_os("POD_NAME") {
        Some(p) => p.into_string().unwrap(),
        None => panic!("You are not in a Kubernetes Environment!"),
    };

    let pod_ip = match env::var_os("POD_IP") {
        Some(pi) => pi.into_string().unwrap(),
        None => panic!("You are not in a Kubernetes Environment!"),
    };

    let pod_namespace = match env::var_os("POD_NAMESPACE") {
        Some(pn) => pn.into_string().unwrap(),
        None => panic!("You are not in a Kubernetes Environment!"),
    };

    let pod_service = match env::var_os("POD_SERVICE") {
        Some(ps) => ps.into_string().unwrap(),
        None => panic!("You are not in a Kubernetes Environment!"),
    };

    let pod_metadata = PodMetadata {
        node: node,
        pod_name: pod,
        pod_ip: pod_ip,
        pod_namespace: pod_namespace,
        pod_service_account: pod_service,
    };

    HttpResponse::Ok().json(pod_metadata)
}

pub async fn get_pokemon(req: HttpRequest) -> HttpResponse {
    // Parse the path parameter from the request URL
    let id: String = req.match_info().get("id").unwrap().parse().unwrap();

    // Set the url
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", id);
    // println!("{}",url);

    // Create a client and send the request
    let client = reqwest::Client::new();
    let response = client.get(url).send().await.unwrap();

    // println!("{:#?}", response);
    // let res_json = response.json::<Pokemon>().await;
    // println!("{:#?}", res_json);

    // Match condition based on status code
    match response.status() {
        reqwest::StatusCode::OK => match response.json::<Pokemon>().await {
            Ok(parsed) => HttpResponse::Ok().json(parsed),
            Err(_) => HttpResponse::BadRequest().json("Something went wrong!"),
        },
        other => {
            panic!("Uh oh something went really wrong! {:?}", other)
        }
    }

}


