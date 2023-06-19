use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

use crate::{models::layouts::LayoutModel, repositories::MongoRepo};

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("layouts");
    cfg.service(
        scope
            .service(get_layouts)
            .service(create_layout)
            .service(get_layout)
            .service(update_layout)
            .service(delete_layout),
    );
}

#[derive(Deserialize)]
struct LayoutPath {
    layout_name: String,
}

#[get("")]
async fn get_layouts(repo: web::Data<MongoRepo>) -> impl Responder {
    let layouts = repo.layouts.find_all().await;

    match layouts {
        Ok(layouts) => HttpResponse::Ok().json(json!({ "data": { "layouts": layouts } })),
        Err(err) => HttpResponse::InternalServerError().json(json!({ "error": err.to_string() })),
    }
}

#[post("")]
async fn create_layout(
    layout: web::Json<LayoutModel>,
    repo: web::Data<MongoRepo>,
) -> impl Responder {
    if layout.name.is_none() {
        return HttpResponse::BadRequest().json(json!({ "error": "Layout name is required" }));
    }
    if layout.keys.len() != 3 {
        return HttpResponse::BadRequest().json(json!({ "error": "Layout keys must have 3 rows" }));
    }
    if layout.keys.iter().any(|row| row.len() != 10) {
        return HttpResponse::BadRequest()
            .json(json!({ "error": "Layout key rows must have 10 keys" }));
    }

    let result = repo.layouts.create(&layout).await;

    match result {
        Ok(data) => HttpResponse::Created().json(json!({ "data": data })),
        Err(err) => HttpResponse::InternalServerError().json(json!({ "error": err.to_string() })),
    }
}

#[get("/{layout_name}")]
async fn get_layout(path: web::Path<LayoutPath>, repo: web::Data<MongoRepo>) -> impl Responder {
    let result = repo.layouts.find(&path.layout_name).await;

    match result {
        Ok(Some(data)) => HttpResponse::Ok().json(json!({ "data": data })),
        Ok(None) => HttpResponse::NotFound().json(json!({ "error": "Layout not found" })),
        Err(err) => HttpResponse::InternalServerError().json(json!({ "error": err.to_string() })),
    }
}

#[put("/{layout_name}")]
async fn update_layout(
    path: web::Path<LayoutPath>,
    layout: web::Json<LayoutModel>,
    repo: web::Data<MongoRepo>,
) -> impl Responder {
    if layout.keys.len() != 3 {
        return HttpResponse::BadRequest().json(json!({ "error": "Layout keys must have 3 rows" }));
    }
    if layout.keys.iter().any(|row| row.len() != 10) {
        return HttpResponse::BadRequest()
            .json(json!({ "error": "Layout key rows must have 10 keys" }));
    }
    let result = repo.layouts.update(&path.layout_name, &layout).await;

    match result {
        Ok(Some(_)) => HttpResponse::NoContent().finish(),
        Ok(None) => HttpResponse::NotFound().json(json!({ "error": "Layout not found" })),
        Err(err) => HttpResponse::InternalServerError().json(json!({ "error": err.to_string() })),
    }
}

#[delete("/{layout_name}")]
async fn delete_layout(path: web::Path<LayoutPath>, repo: web::Data<MongoRepo>) -> impl Responder {
    let result = repo.layouts.delete(&path.layout_name).await;

    match result {
        Ok(Some(_)) => HttpResponse::NoContent().finish(),
        Ok(None) => HttpResponse::NotFound().json(json!({ "error": "Layout not found" })),
        Err(err) => HttpResponse::InternalServerError().json(json!({ "error": err.to_string() })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::test;

    #[actix_web::test]
    async fn test_get_layouts() {
        let app = test::init_service(
            actix_web::App::new()
                .app_data(web::Data::new(MongoRepo::init("test").await))
                .configure(register_routes),
        )
        .await;

        let req = actix_web::test::TestRequest::get()
            .uri("/layouts")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_crud() {
        let app = test::init_service(
            actix_web::App::new()
                .app_data(web::Data::new(MongoRepo::init("test").await))
                .configure(register_routes),
        )
        .await;

        let qwerty = json!({
            "name": "qwertybad",
            "keys": [ "qwertyuiop", "asdfghjkl;", "zxcvbnm,./" ],
        });

        let req = test::TestRequest::post()
            .uri("/layouts")
            .set_json(&qwerty)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success(), "Create layout");

        let qwerty = json!({
            "name": "qwerty",
            "keys": [ "qwertyuiop", "asdfghjkl;", "zxcvbnm,./" ],
        });

        let req = test::TestRequest::put()
            .uri("/layouts/qwertybad")
            .set_json(&qwerty)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success(), "Update layout");

        let req = test::TestRequest::get().uri("/layouts/qwerty").to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success(), "Get layout");

        let req = test::TestRequest::delete()
            .uri("/layouts/qwerty")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success(), "Delete layout");

        let req = test::TestRequest::get().uri("/layouts/qwerty").to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error(), "Get deleted layout");

        MongoRepo::drop("test").await;
    }
}
