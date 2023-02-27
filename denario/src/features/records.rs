use actix_web::{get,post,patch,delete, HttpResponse, Responder, web};
use serde_json::json;
use rusqlite::{ Connection};

use super::super::db_conn::get_db_connection;
use super::super::models::record_model::{Record,DtoRecord,Record2Categories,FiltersRecord};

#[get("/records/find_filtered")]
async fn find_all_records_filtered(filters: web::Query<FiltersRecord>) -> impl Responder {
    
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

#[get("/records/by_category/{id}")]
async fn find_one_record_by_category(path: web::Path<(u64,)>) -> impl Responder {
    let conn=get_db_connection();
    
    let id=path.into_inner().0;
    let sql=format!("SELECT * FROM records WHERE category_id={id} AND is_deleted=0");
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"success": true,"records": result_vec}))
}

#[get("/records/by_date/{month}/{year}")]
async fn find_all_records_by_date(path: web::Path<(u8,u16)>) -> impl Responder {
    let conn=get_db_connection();
    
    let path_params=path.into_inner();
    let month=path_params.0;
    let year=path_params.1;
    
    let month_str:String;
    if month < 10{
        month_str=format!("0{month}");
    }else{
        month_str=format!("{month}");
    }

    let sql=format!("SELECT * FROM records WHERE record_date LIKE \'{year}-{month_str}-%\' AND is_deleted=0");
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"success": true,"records": result_vec}))
}

#[get("/records/by_date/{day}/{month}/{year}")]
async fn find_all_records_by_date_full(path: web::Path<(u8,u8,u16)>) -> impl Responder {
    let conn=get_db_connection();
    
    let path_params=path.into_inner();
    let day=path_params.0;
    let month=path_params.1;
    let year=path_params.2;
    
    let day_str:String;
    if day < 10{
        day_str=format!("0{day}");
    }else{
        day_str=format!("{day}");
    }

    let month_str:String;
    if month < 10{
        month_str=format!("0{month}");
    }else{
        month_str=format!("{month}");
    }

    let sql=format!("SELECT * FROM records WHERE record_date LIKE \'{year}-{month_str}-{day_str}\' AND is_deleted=0");
    println!("{:?}",sql);
    let result_vec=execute_query_and_parse(&conn, &sql);

    HttpResponse::Ok().json(json!({"success": true,"records": result_vec}))
}

#[get("/records/by_category/{id}/date/{month}/{year}")]
async fn find_all_records_by_category_and_date(path: web::Path<(u64,u8,u16)>) -> impl Responder {
    let conn=get_db_connection();
    
    let path_params=path.into_inner();
    let category_id=path_params.0;
    let month=path_params.1;
    let year=path_params.2;
    
    let month_str:String;
    if month < 10{
        month_str=format!("0{month}");
    }else{
        month_str=format!("{month}");
    }

    let sql=format!("SELECT * FROM records WHERE record_date LIKE \'{year}-{month_str}-%\' AND category_id={category_id} AND is_deleted=0");
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

// FUNCIONES ESPECIALES
// trae la suma de todos los in y todos out por fecha.
// trae la suma de todos los in y todos los out discriminado por categoría
// para esto hago un patron build
#[get("/records/amounts_by_date/{month}/{year}")]
async fn find_all_records_amounts_io_by_date(path: web::Path<(u8,u16)>) -> impl Responder {
    let conn=get_db_connection();
    
    let path_params=path.into_inner();
    let month=path_params.0;
    let year=path_params.1;
    
    let month_str:String;
    if month < 10{
        month_str=format!("0{month}");
    }else{
        month_str=format!("{month}");
    }

    let sql=format!("SELECT R.amount_io,R.category_id,C.name AS category_name ,SUM(R.amount) AS total FROM records R LEFT JOIN categories C ON R.category_id=C.id WHERE R.record_date LIKE '{year}-{month_str}-%' AND R.is_deleted=0 GROUP BY R.category_id");
    let result_vec=execute_query_and_parse_record2categories(&conn, &sql);

    HttpResponse::Ok().json(json!({"success": true,"records": result_vec}))
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

fn execute_query_and_parse_record2categories(conn: &Connection, sql:&str) -> Vec<Record2Categories>{
    let mut stmt = conn.prepare(&sql).unwrap();
    let elements_iter = stmt.query_map([], |row| {

        Ok(Record2Categories {
            amount_io:row.get(0)?,
            category_id:row.get(1)?,
            category_name:row.get(2)?,
            total:row.get(3)?
        })
    }).unwrap();

    let mut result_vec=vec![];

    for element in elements_iter {
        result_vec.push(element.unwrap());
    }
    return result_vec;
}

