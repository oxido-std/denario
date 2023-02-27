use actix_web::{get,post,patch,delete, HttpResponse, Responder, web};
use serde::Deserialize;
use serde_json::json;
use rusqlite::{ Connection, NO_PARAMS,};

use super::super::db_conn::get_db_connection;


#[derive(Debug,serde::Serialize)]
struct Credit{
    id:i64,
    name:String,
    comment:String,
    amount:f32,
    payments:u16,
    started_at:String,
    finish_at:String,
    category_id:i64,
    created_at:String,
    updated_at:String,
    is_deleted:bool,
}

#[derive(Debug,Deserialize)]
struct DtoCredit{
    name:String,
    comment:String,
    amount:f32,
    payments:u16,
    started_at:String,
    category_id:i64,
}

#[derive(Debug,Deserialize)]
struct Filters{
    limit:u16,
    skip:u16,
    orderby_column:String,
    orderby:String,
}

#[get("/credits/find_filtered")]
async fn find_all_credits_filtered(filters: web::Query<Filters>) -> impl Responder {
    
    let filters=&filters.into_inner();
    println!("{:?}",filters);
    // println!("{:?}",skip);
    // println!("{:?}",orderby);

    let conn=get_db_connection();
    
    let sql=format!("SELECT * FROM credits WHERE is_deleted=0 ORDER BY {} {} LIMIT {} OFFSET {}",filters.orderby_column.to_string(),filters.orderby.to_string(),filters.limit.to_string(),filters.skip.to_string());
    println!("QUERY: {}",sql);
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"status": "200","credits": result_vec}))
}

#[get("/credits")]
async fn find_all_credits() -> impl Responder {

    let conn=get_db_connection();
    
    let sql="SELECT * FROM credits WHERE is_deleted=0 ORDER BY id DESC";
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"status": "200","credits": result_vec}))
}

#[get("/credits/{id}")]
async fn find_one_credit(path: web::Path<(u32,)>) -> impl Responder {
    let conn=get_db_connection();
    
    let id=path.into_inner().0;
    let sql=format!("SELECT * FROM credits WHERE id= {id} AND is_deleted=0");
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"success": true,"credits": result_vec}))
}

#[post("/credits")]
async fn create_credit(data: web::Json<DtoCredit>) -> impl Responder {
    let conn=get_db_connection();
    
    let data=data.into_inner();
    // insert
    let name=data.name.to_string();
    let comment=data.comment.to_string();
    let amount=data.amount.to_string();
    let payments=data.payments.to_string();
    let started_at=data.started_at.to_string();
    let category_id=data.category_id.to_string();

    let sql=format!("INSERT INTO credits (name,comment,amount,payments,started_at,finish_at,category_id,created_at,updated_at,is_deleted) VALUES (?1,?2,?3,?4,?5,date(?5,'+{} month'),?6,datetime('now'),datetime('now'),false)",payments);
    conn.execute(&sql,&[&name,&comment,&amount,&payments,&started_at,&category_id]);
    
    // get last inserted category
    let last_id= conn.last_insert_rowid();
    let sql=format!("SELECT * FROM credits WHERE id={last_id}");
    let result_vec=execute_query_and_parse(&conn, &sql);
    HttpResponse::Ok().json(json!({"success": true,"credits": result_vec}))
}

#[patch("/credits/{id}")]
async fn update_credit(path: web::Path<(u32,)>, data: web::Json<DtoCredit>) -> impl Responder {
    let conn=get_db_connection();

    let id=path.into_inner().0;
    let data=data.into_inner();
    // insert
    let name=data.name.to_string();
    let comment=data.comment.to_string();
    let amount=data.amount.to_string();
    let payments=data.payments.to_string();
    let started_at=data.started_at.to_string();
    let category_id=data.category_id.to_string();

    // update
    let sql=format!("UPDATE credits SET name=?1,comment=?2,amount=?3,payments=?4,started_at=?5,finish_at=date(?5,'+{} month'),category_id=?6,updated_at=datetime('now') WHERE id={}",payments,id);
    conn.execute(&sql,&[&name,&comment,&amount,&payments,&started_at,&category_id]);

    // select updated category
    let sql=format!("SELECT * FROM credits WHERE id={id}");
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"success": true,"credits": result_vec}))
}
#[delete("/credits/{id}")]
async fn delete_credit(path: web::Path<(u32,)>) -> impl Responder {
    let conn=get_db_connection();

    let id=path.into_inner().0;

    // update
    let sql=format!("UPDATE credits SET is_deleted=1, updated_at=datetime('now') WHERE id={}",id);
    conn.execute(&sql,NO_PARAMS);

    HttpResponse::Ok().json(json!({"success": true,"deleted": id}   ))
}

///
/// this function make a preperare statemen and execute a query.
/// Then the results is parsed into a vector to serialize as json
/// 

fn execute_query_and_parse(conn: &Connection, sql:&str) -> Vec<Credit>{
    let mut stmt = conn.prepare(&sql).unwrap();
    let elements_iter = stmt.query_map([], |row| {
        Ok(Credit {
            id: row.get(0)?,
            name: row.get(1)?,
            comment: row.get(2)?,
            amount: row.get(3)?,
            payments: row.get(4)?,
            started_at: row.get(5)?,
            finish_at: row.get(6)?,
            category_id: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
            is_deleted: row.get(10)?,
        })
    }).unwrap();

    let mut result_vec=vec![];

    for element in elements_iter {
        result_vec.push(element.unwrap());
    }
    return result_vec;
}