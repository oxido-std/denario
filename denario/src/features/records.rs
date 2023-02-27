use actix_web::{get,post,patch,delete, HttpResponse, Responder, web};
use serde::Deserialize;
use serde_json::json;
use rusqlite::{ Connection};

use super::super::db_conn::get_db_connection;


#[derive(Debug,serde::Serialize)]
struct Record{
    id:i64,
    name:String,
    amount:f32,
    amount_io:String, // in / out
    comment:String,
    record_date:String,
    category_id:i64,
    created_at:String,
    updated_at:String,
    is_deleted:bool,
}

#[derive(Debug,Deserialize)]
struct DtoRecord{
    name:String,
    amount:f32,
    amount_io:String,
    comment:String,
    record_date:String,
    category_id:i64,
}

#[derive(Debug,Deserialize)]
struct Filters{
    limit:u16,
    skip:u16,
    orderby_column:String,
    orderby:String,
}

#[get("/records/find_filtered")]
async fn find_all_records_filtered(filters: web::Query<Filters>) -> impl Responder {
    
    let filters=&filters.into_inner();

    let conn=get_db_connection();
    
    let sql=format!("SELECT * FROM records WHERE is_deleted=0 ORDER BY {} {} LIMIT {} OFFSET {}",filters.orderby_column.to_string(),filters.orderby.to_string(),filters.limit.to_string(),filters.skip.to_string());
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"status": "200","records": result_vec}))
}

#[get("/records")]
async fn find_all_records() -> impl Responder {

    let conn=get_db_connection();
    
    let sql="SELECT * FROM records WHERE is_deleted=0 ORDER BY id DESC";
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"status": "200","records": result_vec}))
}

#[get("/records/{id}")]
async fn find_one_record(path: web::Path<(u32,)>) -> impl Responder {
    let conn=get_db_connection();
    
    let id=path.into_inner().0;
    let sql=format!("SELECT * FROM records WHERE id= {id} AND is_deleted=0");
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"success": true,"records": result_vec}))
}

#[post("/records")]
async fn create_record(data: web::Json<DtoRecord>) -> impl Responder {
    let conn=get_db_connection();
    
    let data=data.into_inner();
    // insert
    let name=data.name.to_string();
    let amount=data.amount.to_string();
    let amount_io=data.amount_io.to_string();
    let comment=data.comment.to_string();
    let record_date=data.record_date.to_string();
    let category_id=data.category_id.to_string();

    let sql="INSERT INTO records (name,amount,amount_io,comment,record_date,category_id,created_at,updated_at,is_deleted) VALUES (?1,?2,?3,?4,?5,?6,datetime('now'),datetime('now'),false)";
    let _ =conn.execute(&sql,&[&name,&amount,&amount_io,&comment,&record_date,&category_id]);
    
    // get last inserted category
    let last_id= conn.last_insert_rowid();
    let sql=format!("SELECT * FROM records WHERE id={last_id}");
    let result_vec=execute_query_and_parse(&conn, &sql);
    HttpResponse::Ok().json(json!({"success": true,"records": result_vec}))
}

#[patch("/records/{id}")]
async fn update_record(path: web::Path<(u32,)>, data: web::Json<DtoRecord>) -> impl Responder {
    let conn=get_db_connection();

    let id=path.into_inner().0;
    let data=data.into_inner();
    // insert
    let name=data.name.to_string();
    let amount=data.amount.to_string();
    let amount_io=data.amount_io.to_string();
    let comment=data.comment.to_string();
    let record_date=data.record_date.to_string();
    let category_id=data.category_id.to_string();

    // update
    let sql=format!("UPDATE records SET name=?1,amount=?2,amount_io=?3,comment=?4,record_date=?5,category_id=?6,updated_at=datetime('now') WHERE id={}",id);
    let _ =conn.execute(&sql,&[&name,&amount,&amount_io,&comment,&record_date,&category_id]);

    // select updated category
    let sql=format!("SELECT * FROM records WHERE id={id}");
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"success": true,"records": result_vec}))
}
#[delete("/records/{id}")]
async fn delete_record(path: web::Path<(u32,)>) -> impl Responder {
    let conn=get_db_connection();

    let id=path.into_inner().0;

    // update
    let sql=format!("UPDATE records SET is_deleted=1, updated_at=datetime('now') WHERE id={}",id);
    let _ =conn.execute(&sql,[]);

    HttpResponse::Ok().json(json!({"success": true,"deleted": id}   ))
}

///
/// this function make a preperare statemen and execute a query.
/// Then the results is parsed into a vector to serialize as json
/// 

fn execute_query_and_parse(conn: &Connection, sql:&str) -> Vec<Record>{
    let mut stmt = conn.prepare(&sql).unwrap();
    let elements_iter = stmt.query_map([], |row| {
        Ok(Record {
            id: row.get(0)?,
            name: row.get(1)?,
            amount: row.get(2)?,
            amount_io: row.get(3)?,
            comment: row.get(4)?,
            record_date: row.get(5)?,
            category_id: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
            is_deleted: row.get(9)?,
        })
    }).unwrap();

    let mut result_vec=vec![];

    for element in elements_iter {
        result_vec.push(element.unwrap());
    }
    return result_vec;
}