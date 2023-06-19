use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    models::{bigrams::ProcessTextRequest, pagination::Pagination},
    repositories::MongoRepo,
};

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(process_text).service(get_process_text);
}

#[post("/process_text")]
async fn process_text(
    data: web::Json<ProcessTextRequest>,
    repo: web::Data<MongoRepo>,
) -> impl Responder {
    let text = data
        .text
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .collect::<String>()
        .to_lowercase();
    let words = text.split_whitespace().collect::<Vec<&str>>();

    println!("words: {:?}", words);

    let mut bigram_count = 0;
    for pair in words.windows(2) {
        println!("pair: {:?}", pair);
        let result = repo.bigrams.upsert(pair[0], pair[1]).await;
        match result {
            Ok(_) => bigram_count += 1,
            Err(err) => {
                return HttpResponse::InternalServerError().json(json!({
                    "error": err.to_string()
                }));
            }
        }
    }

    HttpResponse::Ok().json(json!({ "data": { "bigram_count": bigram_count } }))
}

#[get("/process_text")]
async fn get_process_text(
    repo: web::Data<MongoRepo>,
    query: web::Query<Pagination>,
) -> impl Responder {
    let result = repo.bigrams.find_all(query.into_inner()).await;

    match result {
        Ok(data) => {
            HttpResponse::Ok().json(json!({ "data": { "bigrams": data, "count": data.len() } }))
        }
        Err(err) => HttpResponse::InternalServerError().json(json!({ "error": err.to_string() })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::test;

    #[actix_web::test]
    async fn test_process_text() {
        let app = test::init_service(
            actix_web::App::new()
                .app_data(web::Data::new(MongoRepo::init("test").await))
                .configure(register_routes),
        )
        .await;

        let body = json!({ "text": "Hello world and hello everyone, this is a test of the process_text endpoint" });

        let req = actix_web::test::TestRequest::post()
            .uri("/process_text")
            .set_json(&body)
            .to_request();

        let resp = actix_web::test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        MongoRepo::drop("test").await;
    }
}
