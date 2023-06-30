use actix_cors::Cors;
use actix_web::{middleware::Logger, http};
use actix_files as fs;
use actix_web::{App, HttpServer };
use dotenv::dotenv;
use std::env;

mod db_conn;
mod features;
pub mod models;
pub mod schema;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // app info
    const APP_NAME: &str = env!("CARGO_PKG_NAME");
    const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

    dotenv().ok();

    let server_host=env::var("SERVER_HOST").expect("SERVER_HOST must be set");
    let server_port=env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    let server_port=server_port.parse::<u16>().expect("SERVER_PORT must be a number");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🦀-----------------------------------------------------------🦀");
    println!("                  🪙  {} [{}]",APP_NAME,APP_VERSION);
    println!("   🚀 Server started successfully at http://{}:{}",server_host,server_port);
    println!("   🔗 View in webbrowser at http://{}:{}/",server_host,server_port);
    println!("🦀-----------------------------------------------------------🦀");

    HttpServer::new(move || {
        // CORS
        let cors = Cors::default()
              .allowed_origin("http://localhost:8080")
              .allowed_origin_fn(|origin, _req_head| {
                  origin.as_bytes().ends_with(b"8080")
              })
              .allowed_methods(vec!["GET", "POST","PATCH","DELETE"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);
        // app_setup
        // let app_setup = features::app_setup::AppSetup::new();
        // APP 
        App::new()
            .wrap(cors)
            // app_setup
            .service(features::app_setup::run_setup)
            // STATIC
            .service(fs::Files::new("/","./public").index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind((server_host, server_port))?
    .run()
    .await
}


pub fn run_setup(){
    dotenv().ok();
    let db_conn = db_conn::establish_connection();
    let app_setup_eneabled = env::var("APP_SETUP").expect("APP_SETUP must be set") == "true";
    let app_setup = features::app_setup::AppSetup { db_conn, app_setup_eneabled };
    app_setup.main();
}