use actix_web::{Responder, HttpResponse, get};

use dotenv::dotenv;
use std::{env};

struct AppStatus {
    pub app_name: String,
    pub app_version: String,
    pub server_host: String,
    pub server_port: String,
    pub app_setup_eneabled: bool,
}

impl AppStatus {
    pub fn new() -> Self{
        dotenv().ok();
        let app_name = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME must be set");
        let app_version = env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION must be set");
        let server_host = env::var("SERVER_HOST").expect("SERVER_HOST must be set");
        let server_port = env::var("SERVER_PORT").expect("SERVER_PORT must be set");
        let app_setup_eneabled = env::var("APP_SETUP_ENABLED").expect("APP_SETUP must be set") == "true";
        return Self { app_name, app_version, server_host, server_port, app_setup_eneabled }
    }
}

#[get("/app_check_status")]
pub async fn app_check_status() -> impl Responder {
    let mut response=format!("App check status");
    // let app_status = AppStatus::new();
    HttpResponse::Ok().json({
        response
    })
}