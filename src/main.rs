use actix_web::{middleware, web, App, HttpServer};
use log::info;

mod controllers;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let bind_address = std::env::var("BIND_ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("PORT")
        .unwrap_or("8000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    info!("Starting server on {bind_address}:{port}");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .configure(controllers::register_routes)
            .route("/", web::get().to(handlers::index))
            .default_service(web::route().to(handlers::not_found))
    })
    .bind((bind_address, port))?
    .run()
    .await
}
