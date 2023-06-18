use actix_web::web;

pub mod examples;
pub mod greetings;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1");

    cfg.service(
        scope
            .configure(examples::register_routes)
            .configure(greetings::register_routes),
    );
}
