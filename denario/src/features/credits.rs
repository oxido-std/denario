use std::fmt::format;

use actix_web::{get,post,patch,delete, HttpResponse, Responder, web};
use serde_json::json;
use chrono::{self, NaiveDate, Datelike};
use rusqlite::{ Connection};

use crate::models::credit_model::SQLCredit;
use crate::models::record_model::SQLRecord;

use super::super::db_conn::get_db_connection;
use super::super::models::credit_model::{Credit,DtoCredit,FiltersCredit};
use super::super::models::record_model::{Record,DtoRecord};



#[get("/credits/find_filtered")]
async fn find_all_credits_filtered(filters: web::Query<FiltersCredit>) -> impl Responder {
    
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
    let amount=data.amount;
    let payments=data.payments;
    let started_at=data.started_at.to_string();
    let category_id=data.category_id;

    let sql=Credit::get_query_insert(&payments.to_string());
    let _ =conn.execute(&sql,&[&name,&comment,&amount.to_string(),&payments.to_string(),&started_at,&category_id.to_string()]);

    
    // get last inserted category
    let sql=Credit::get_last_credit_id();
    let result_vec=execute_query_and_parse(&conn, &sql);
    
    let credit_id=result_vec[0].id;
    // aquí debería hacer un insert en la records por cada cuota del crédito.
    create_record_2_credit(&conn, &name, amount, payments,&started_at,category_id,credit_id);
    
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
    let sql=Credit::get_query_update(&payments, id);
    let _ =conn.execute(&sql,&[&name,&comment,&amount,&payments,&started_at,&category_id]);

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
    let sql=Credit::get_query_delete(id);
    let _ =conn.execute(&sql,[]);

    // debería borrar cada uno de los regitros asociados a un crédito.
    todo!();

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

fn create_record_2_credit(conn:&Connection,name:&str,amount:f32,payments:u16,first_payment_date:&String,category_id:i64,credit_id:i64){
    
    if first_payment_date.starts_with("-"){
        // obtengo la fecha
        let date=chrono::offset::Utc::now();
        
        let day=date.day();
        let mut month=date.month();

        let day_str:String;
        if day < 10{
            day_str=format!("0{day}");
        }else{
            day_str=format!("{day}");
        }

        let month_str:String;
        // si el día es mayor a 25 entoces pago la primer cuota el próximo mes, sino la pago este
        if day > 25{
            // siempre y cuando el mes no sea diciembre lo incremento
            if month != 12{
                month+=1;
            }else{
                month=1; // es enero
            }
        }
        if month < 10{
            month_str=format!("0{month}");
        }else{
            month_str=format!("{month}");
        }

        let year=date.year().to_string();
        let first_payment_date=format!("{year}-{month_str}-{day_str}");
    }

    for i in 1..payments+1{
        let current_payment=i.try_into().unwrap();
        insert_record_2_credit(conn, name, amount, current_payment, payments,&first_payment_date,category_id,credit_id);
    }
}

fn insert_record_2_credit(conn:&Connection,name:&str,amount:f32,current_payment:u8,payments:u16,fist_paymente_date:&String,category_id:i64,credit_id:i64){

    let new_record=DtoRecord{
        name:format!("_CREDITO_ {}",name),
        amount:amount,
        amount_io:String::from("out"),
        comment:format!("_CID{credit_id}_ CUOTA: {current_payment}/{payments}"),
        record_date:fist_paymente_date.to_string(),
        category_id:category_id,
    };

    let sql=Record::get_query_insert_future(current_payment);
    // println!("{:?}",sql);
    // println!("{:?}",new_record);
    let _ =conn.execute(&sql,&[&new_record.name,&amount.to_string(),&new_record.amount_io,&new_record.comment,&new_record.record_date,&new_record.category_id.to_string()]);
}