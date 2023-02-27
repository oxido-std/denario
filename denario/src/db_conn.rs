use rusqlite::{ Connection};
use std::env;

pub fn get_db_connection() -> Connection{
    let db=env::var("DB_NAME").unwrap();
    return Connection::open(db).unwrap();
}