use actix_web::{get,post,patch,delete, HttpResponse, Responder, web};
use serde::Deserialize;
use serde_json::json;
use rusqlite::{ Connection, NO_PARAMS,};

use super::super::db_conn::get_db_connection;


#[derive(Debug,serde::Serialize)]
struct Dolar{
    id:i64,
    name:String,
    amount:f32,
    source:String,
    created_at:String,
    is_deleted:bool,
}

#[derive(Debug,Deserialize)]
struct DtoDolar{
    name:String,
    amount:f32,
    source:String,
}

#[get("/dolars")]
async fn find_all_dolars() -> impl Responder {
    
    let conn=get_db_connection();
    
    let sql="SELECT * FROM dolars WHERE is_deleted=0";
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"status": "200","dolars": result_vec}))
}

#[get("/dolars/{id}")]
async fn find_one_dolar(path: web::Path<(u32,)>) -> impl Responder {
    let conn=get_db_connection();
    
    let id=path.into_inner().0;
    let sql=format!("SELECT * FROM dolars WHERE id= {id} AND is_deleted=0");
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"success": true,"dolars": result_vec}))
}

#[get("/dolars/get_last/{name}")]
async fn get_last_dolar(path: web::Path<(String,)>) -> impl Responder {
    let conn=get_db_connection();
    
    let name=path.into_inner().0;
    let name=name.to_string().to_uppercase();
    let sql=format!("SELECT * FROM dolars WHERE name LIKE '{name}' AND is_deleted=0 ORDER BY id DESC LIMIT 1");
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"success": true,"dolars": result_vec}))
}

#[post("/dolars")]
async fn create_dolar(data: web::Json<DtoDolar>) -> impl Responder {
    let conn=get_db_connection();
    
    let data=data.into_inner();
    // insert
    let name=data.name.to_string().to_uppercase();
    let amount=data.amount.to_string();
    let source=data.source.to_string();
    
    let sql="INSERT INTO dolars (name,amount,source,created_at,is_deleted) VALUES (?1,?2,?3,datetime('now'),false)";
    conn.execute(&sql,&[&name,&amount,&source]);
    
    // get last inserted category
    let last_id= conn.last_insert_rowid();
    let sql=format!("SELECT * FROM dolars WHERE id={last_id}");
    let result_vec=execute_query_and_parse(&conn, &sql);
    HttpResponse::Ok().json(json!({"success": true,"dolars": result_vec}))
}

#[delete("/dolars/{id}")]
async fn delete_dolar(path: web::Path<(u32,)>) -> impl Responder {
    let conn=get_db_connection();

    let id=path.into_inner().0;

    // update
    let sql=format!("UPDATE dolars SET is_deleted=1, updated_at=datetime('now') WHERE id={}",id);
    conn.execute(&sql,NO_PARAMS);

    HttpResponse::Ok().json(json!({"success": true,"deleted": id}   ))
}

///
/// this function make a preperare statemen and execute a query.
/// Then the results is parsed into a vector to serialize as json
/// 

fn execute_query_and_parse(conn: &Connection, sql:&str) -> Vec<Dolar>{
    let mut stmt = conn.prepare(&sql).unwrap();
    let elements_iter = stmt.query_map([], |row| {
        Ok(Dolar {
            id: row.get(0)?,
            name: row.get(1)?,
            amount: row.get(2)?,
            source: row.get(3)?,
            created_at: row.get(4)?,
            is_deleted: row.get(5)?,
        })
    }).unwrap();

    let mut result_vec=vec![];

    for element in elements_iter {
        result_vec.push(element.unwrap());
    }
    return result_vec;
}