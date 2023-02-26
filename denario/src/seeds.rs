use rusqlite::{ Connection, NO_PARAMS };
use dotenv::dotenv;
use std::env;

pub fn setup() {
    
    let db=env::var("DB_NAME").unwrap();
    let conn = Connection::open(db).unwrap();
    
    conn.execute("CREATE TABLE records (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(50) NOT NULL,
        amount FLOAT NOT NULL,
        type BOOLEAN NOT NULL,
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
        amount FLOAT NOT NULL,
        payments INTEGER NOT NULL,
        started_at TEXT NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NULL,
        is_deleted BOOLEAN NOT NULL
      )", NO_PARAMS).unwrap();
    
}

pub fn populate_categories(){

  let db=env::var("DB_NAME").unwrap();
  let conn = Connection::open(db).unwrap();

  let categories_arr=["Arreglos","Bazar","Comida","Crédito","Educación","Extra","Juegos","Ropa","Serv. de Agua","Serv. de Electricidad","Serv. de Gas","Serv. de Internet","Servicio","SUBE","Sueldo","Suscripción","Teléfono","Transporte"];

  for cat in categories_arr{
    conn.execute("INSERT INTO categories (name,created_at,updated_at,is_deleted) VALUES (?1,datetime('now'),datetime('now'),false)",
    &[&cat.to_string()]).unwrap();
  }


}