use actix_web::{get,HttpResponse, Responder};
use rusqlite::NO_PARAMS;
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
        is_deleted BOOLEAN NOT NULL
      )", NO_PARAMS).unwrap();

      conn.execute("CREATE TABLE categories (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(50) NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NULL,
        is_deleted BOOLEAN NOT NULL
      )", NO_PARAMS).unwrap();

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
      )", NO_PARAMS).unwrap();
      
      conn.execute("CREATE TABLE dolars (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(50) NOT NULL,
        amount FLOAT NOT NULL,
        source VARCHAR(255) NOT NULL,
        created_at TEXT NOT NULL,
        is_deleted BOOLEAN NOT NULL
      )", NO_PARAMS).unwrap();
    
}

pub fn populate_categories(){

  let conn=get_db_connection();

  let categories_arr=["Arreglos","Bazar","Comida","Crédito","Educación","Extra","Juegos","Ropa","Serv. de Agua","Serv. de Electricidad","Serv. de Gas","Serv. de Internet","Servicio","SUBE","Sueldo","Suscripción","Teléfono","Transporte"];

  for cat in categories_arr{
    conn.execute("INSERT INTO categories (name,created_at,updated_at,is_deleted) VALUES (?1,datetime('now'),datetime('now'),false)",
    &[&cat.to_string()]).unwrap();
  }

}

// pub fn populate_records(){
//   let conn=get_db_connection();

//   let records_arr=["Arreglos","Bazar","Comida","Crédito","Educación","Extra","Juegos","Ropa","Serv. de Agua","Serv. de Electricidad","Serv. de Gas","Serv. de Internet","Servicio","SUBE","Sueldo","Suscripción","Teléfono","Transporte"];

//   for elem in records_arr{
//     conn.execute("INSERT INTO records (name,amount,amount_io,comment,record_date,category_id,created_at,updated_at,is_deleted) VALUES (?1,?2,?3,?4,?5,?6,datetime('now'),datetime('now'),false)",
//     &[&elem.to_string()]).unwrap();
//   }
// }

// pub fn populate_credits(){
//   let conn=get_db_connection();

//   let credits_arr=["Arreglos","Bazar","Comida","Crédito","Educación","Extra","Juegos","Ropa","Serv. de Agua","Serv. de Electricidad","Serv. de Gas","Serv. de Internet","Servicio","SUBE","Sueldo","Suscripción","Teléfono","Transporte"];

//   for elem in credits_arr{
//     conn.execute("INSERT INTO credits (name,amount,payments,started_at,created_at,updated_at,is_deleted) VALUES (?1,?2,?3,?4,datetime('now'),datetime('now'),false)",
//     &[&elem.to_string()]).unwrap();
//   }

// }