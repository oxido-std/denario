use actix_web::{get,HttpResponse, Responder};
use serde_json::json;

use super::db_conn::get_db_connection;

#[get("/seeds/setup")]
pub async fn req_seed_setup() -> impl Responder {
    setup();
    populate_categories();
    const MESSAGE: &str = "Creating DB with Tables & populate tables";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

pub fn setup() {

    let conn=get_db_connection();
    
    conn.execute("CREATE TABLE records (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(50) NOT NULL,
        amount FLOAT NOT NULL,
        amount_io VARCHAR(3) NOT NULL,
        comment TEXT NULL,
        record_date TEXT NOT NULL,
        category_id INTEGER NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NULL,
        is_deleted BOOLEAN NOT NULL,
        is_mutable BOOLEAN NOT NULL
      )", []).unwrap();

      conn.execute("CREATE TABLE categories (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(50) NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NULL,
        is_deleted BOOLEAN NOT NULL
      )", []).unwrap();

      conn.execute("CREATE TABLE credits (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(50) NOT NULL,
        comment TEXT NULL,
        amount FLOAT NOT NULL,
        payments INTEGER NOT NULL,
        started_at TEXT NOT NULL,
        finish_at TEXT NOT NULL,
        category_id INTEGER NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NULL,
        is_deleted BOOLEAN NOT NULL
      )", []).unwrap();
      
      conn.execute("CREATE TABLE dolars (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(50) NOT NULL,
        amount FLOAT NOT NULL,
        source VARCHAR(255) NOT NULL,
        created_at TEXT NOT NULL,
        is_deleted BOOLEAN NOT NULL
      )", []).unwrap();
    
}

pub fn populate_categories(){

  let conn=get_db_connection();

  let categories_arr=["Arreglos De La Casa","Bazar","Comida","Crédito","Educación","Extra","Juegos","Ropa","Serv. De Agua","Serv. De Electricidad","Serv. De Gas","Serv. De Internet","Servicio","Sueldo","Suscripción","Teléfono","Transporte","Libros","Salud Mental","Salud","Monotributo","Impuestos","_random","Informática"];

  for cat in categories_arr{
    conn.execute("INSERT INTO categories (name,created_at,updated_at,is_deleted) VALUES (?1,datetime('now'),datetime('now'),false)",
    &[&cat.to_string()]).unwrap();
  }

}