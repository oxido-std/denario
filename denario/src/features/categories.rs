use actix_web::{get,post,patch,delete, HttpResponse, Responder, web};
use serde_json::json;
use rusqlite::{ Connection};

use crate::models::category_model::SQLCategory;

use super::super::db_conn::get_db_connection;
use super::super::models::category_model::{Category,DtoCategory};



#[get("/categories")]
async fn find_all_categories() -> impl Responder {
    
    let conn=get_db_connection();
    
    let sql="SELECT * FROM categories WHERE is_deleted=0";
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"status": "200","categories": result_vec}))
}

#[get("/categories/{id}")]
async fn find_one_category(path: web::Path<(u32,)>) -> impl Responder {
    let conn=get_db_connection();
    
    let id=path.into_inner().0;
    let sql=format!("SELECT * FROM categories WHERE id= {id} AND is_deleted=0");
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"success": true,"categories": result_vec}))
}

#[post("/categories")]
async fn create_category(data: web::Json<DtoCategory>) -> impl Responder {
    let conn=get_db_connection();
    
    // insert
    let name=data.into_inner().name;
    
    let sql=Category::get_query_insert();
    let _ =conn.execute(&sql,&[&name.to_string()]);
    
    // get last inserted category
    let last_id= conn.last_insert_rowid();
    let sql=format!("SELECT * FROM categories WHERE id={last_id}");
    let result_vec=execute_query_and_parse(&conn, &sql);
    HttpResponse::Ok().json(json!({"success": true,"categories": result_vec}))
}

#[patch("/categories/{id}")]
async fn update_category(path: web::Path<(u32,)>, data: web::Json<DtoCategory>) -> impl Responder {
    let conn=get_db_connection();

    let id=path.into_inner().0;
    let name=data.into_inner().name;

    // update
    let sql=Category::get_query_update(id);
    let _ =conn.execute(&sql,&[&name.to_string()]);

    // select updated category
    let sql=format!("SELECT * FROM categories WHERE id={id}");
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"success": true,"categories": result_vec}))
}
#[delete("/categories/{id}")]
async fn delete_category(path: web::Path<(u32,)>) -> impl Responder {
    let conn=get_db_connection();

    let id=path.into_inner().0;

    // update
    let sql=Category::get_query_delete(id);
    let _ =conn.execute(&sql,[]);

    HttpResponse::Ok().json(json!({"success": true,"deleted": id}   ))
}

///
/// this function make a preperare statemen and execute a query.
/// Then the results is parsed into a vector to serialize as json
/// 

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