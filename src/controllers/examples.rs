use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
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
    async fn test_ping() {
        let app = test::init_service(actix_web::App::new().configure(register_routes)).await;

        let req = actix_web::test::TestRequest::get()
            .uri("/ping")
            .to_request();

        let resp = actix_web::test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = actix_web::test::read_body(resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(body, r#"{"message":"pong"}"#);
    }
}
