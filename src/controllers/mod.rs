use actix_web::web;

pub mod bigrams;
pub mod examples;
pub mod layouts;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1");

    cfg.service(
        scope
            .configure(examples::register_routes)
            .configure(bigrams::register_routes)
            .configure(layouts::register_routes),
    );
}
