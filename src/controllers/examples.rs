use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("examples");
    cfg.service(scope.service(greet).service(ping));
}

#[derive(Deserialize)]
struct GreetingPath {
    username: String,
}

#[get("/greetings/{username}")]
async fn greet(path: web::Path<GreetingPath>) -> impl Responder {
    let greeting = format!("Hello {}!", path.username);
    HttpResponse::Ok().json(json!({ "message": greeting }))
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "pong" }))
}
