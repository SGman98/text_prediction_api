use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use log::info;

mod controllers;
mod handlers;
mod models;
mod repositories;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let bind_address = std::env::var("BIND_ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("PORT")
        .unwrap_or("8000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");
    let front_url = std::env::var("FRONT_URL").expect("FRONT_URL must be set");

    let db_name = std::env::var("MONGO_DB").expect("MONGO_DB must be set");
    let repo = repositories::MongoRepo::init(&db_name).await;

    info!("Starting server on {bind_address}:{port}");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&front_url)
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(repo.clone()))
            .configure(controllers::register_routes)
            .route("/", web::get().to(handlers::index))
            .default_service(web::route().to(handlers::not_found))
    })
    .bind((bind_address, port))?
    .run()
    .await
}
