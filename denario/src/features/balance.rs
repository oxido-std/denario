use actix_web::{get,post,patch,delete, HttpResponse, Responder, web};
use serde_json::json;
use rusqlite::{ Connection};

use crate::models::category_model::SQLCategory;

use super::super::db_conn::get_db_connection;
use super::super::models::category_model::{Category,DtoCategory};


/// Balance es una clase de consulta para el fronend
/// Trae los balances del mes.
/// 

#[get("/categories")]
async fn find_all_categories() -> impl Responder {
    
    let conn=get_db_connection();
    
    let sql="SELECT * FROM categories WHERE is_deleted=0";
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"status": "200","categories": result_vec}))
}


fn execute_query_and_parse(conn: &Connection, sql:&str) -> Vec<Category>{
    let mut stmt = conn.prepare(&sql).unwrap();
    let elements_iter = stmt.query_map([], |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
            updated_at: row.get(3)?,
            is_deleted: row.get(4)?,
        })
    }).unwrap();

    let mut result_vec=vec![];

    for element in elements_iter {
        result_vec.push(element.unwrap());
    }
    return result_vec;
}