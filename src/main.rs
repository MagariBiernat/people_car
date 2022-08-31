use actix_web::{ App, HttpServer, middleware::Logger, HttpResponse, http::StatusCode, Responder, web};
mod services;
mod config;

use dotenv::dotenv;

async fn not_found() -> impl Responder {
    HttpResponse::build(StatusCode::NOT_FOUND)
        .body("Route not found - 404")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    println!("Server started at host {}:{}", config.server.host, config.server.port);


    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(services::api::calculate_diesel_usage_for_distance)
            .service(services::api::probability_of_unit_ijector_fail)
            .default_service(
                web::route().to(not_found)
            )
            
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
