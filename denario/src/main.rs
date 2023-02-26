use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use dotenv::dotenv;
use std::env;

mod seeds;
mod features;

#[get("/seeds/setup")]
async fn seed_setup() -> impl Responder {
    seeds::setup();
    seeds::populate_categories();
    const MESSAGE: &str = "Creating DB with Tables & populate tables";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust, SQLX, SQLITE,and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let server_host=env::var("SERVER_HOST").unwrap();
    let server_port=env::var("SERVER_PORT").unwrap();
    let server_port=server_port.parse::<u16>().unwrap();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully at http://{}:{}",server_host,server_port);

    HttpServer::new(move || {
        App::new()
            .service(health_checker_handler)
            .service(seed_setup)
            .service(features::categories::find_all_categories)
            .service(features::categories::find_one_category)
            .service(features::categories::create_category)
            .service(features::categories::update_category)
            .service(features::categories::delete_category)
            .wrap(Logger::default())
    })
    .bind((server_host, server_port))?
    .run()
    .await
}
