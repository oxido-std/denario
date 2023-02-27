use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use dotenv::dotenv;
use std::env;

mod seeds;
mod features;
mod db_conn;


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
            .service(seeds::req_seed_setup)
            .service(features::categories::find_all_categories)
            .service(features::categories::find_one_category)
            .service(features::categories::create_category)
            .service(features::categories::update_category)
            .service(features::categories::delete_category)
            .service(features::records::find_all_records_filtered)
            .service(features::records::find_all_records)
            .service(features::records::find_one_record)
            .service(features::records::create_record)
            .service(features::records::update_record)
            .service(features::records::delete_record)
            .service(features::credits::find_all_credits_filtered)
            .service(features::credits::find_all_credits)
            .service(features::credits::find_one_credit)
            .service(features::credits::create_credit)
            .service(features::credits::update_credit)
            .service(features::credits::delete_credit)
            .service(features::dolars::find_all_dolars)
            .service(features::dolars::find_one_dolar)
            .service(features::dolars::get_last_dolar)
            .service(features::dolars::create_dolar)
            .service(features::dolars::delete_dolar)
            .wrap(Logger::default())
    })
    .bind((server_host, server_port))?
    .run()
    .await
}
