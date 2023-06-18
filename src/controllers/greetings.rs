use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

use crate::repositories::MongoRepo;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("greetings");
    cfg.service(scope.service(greet));
}

#[derive(Deserialize)]
struct GreetingPath {
    username: String,
}

#[get("/{username}")]
async fn greet(path: web::Path<GreetingPath>, repo: web::Data<MongoRepo>) -> impl Responder {
    let greeting = repo.greetings.greet(&path.username).await;

    match greeting {
        Ok(Some(greeting)) => HttpResponse::Ok().json(json!({
            "message":
                format!(
                    "Hello {}, you have been greeted {} times",
                    greeting.username,
                    greeting.count + 1
                )
        })),
        Ok(None) => {
            HttpResponse::Ok().json(json!({ "message": format!("Hello {}", path.username) }))
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(json!({ "message": "Failed to greet user" }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::test;

    #[actix_web::test]
    async fn test_greet() {
        let app = test::init_service(
            actix_web::App::new()
                .app_data(web::Data::new(MongoRepo::init("test").await))
                .configure(register_routes),
        )
        .await;

        let req = actix_web::test::TestRequest::get()
            .uri("/greetings/John")
            .to_request();

        let resp = actix_web::test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = actix_web::test::read_body(resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(body, r#"{"message":"Hello John"}"#);

        let req = actix_web::test::TestRequest::get()
            .uri("/greetings/John")
            .to_request();

        let resp = actix_web::test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = actix_web::test::read_body(resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(
            body,
            r#"{"message":"Hello John, you have been greeted 2 times"}"#
        );

        MongoRepo::drop("test").await;
    }
}
