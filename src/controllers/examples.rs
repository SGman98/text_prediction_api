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

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::test;

    #[actix_web::test]
    async fn test_greet() {
        let app = test::init_service(actix_web::App::new().configure(register_routes)).await;

        let req = actix_web::test::TestRequest::get()
            .uri("/examples/greetings/John")
            .to_request();

        let resp = actix_web::test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = actix_web::test::read_body(resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(body, r#"{"message":"Hello John!"}"#);
    }

    #[actix_web::test]
    async fn test_ping() {
        let app = test::init_service(actix_web::App::new().configure(register_routes)).await;

        let req = actix_web::test::TestRequest::get()
            .uri("/examples/ping")
            .to_request();

        let resp = actix_web::test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = actix_web::test::read_body(resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(body, r#"{"message":"pong"}"#);
    }
}
