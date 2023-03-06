use actix_web::{get, HttpResponse, Responder, web};
use serde_json::json;
use rusqlite::{ Connection};


use super::super::db_conn::get_db_connection;
use super::super::models::balance::{Balance,BalanceSum};


/// Balance es una clase de consulta para el fronend
/// Trae los balances del mes seleccionado
/// 
/// Además tiene unas subconsultas para el dash


/// Get all the amounts by amout_in (in/out) With categories by month and year

#[get("/balances/total_amount_with_categories_by/{amount_io}/{month}/{year}")]
pub async fn find_amouts_io_by_month(path: web::Path<(String,u8,u16)>) -> impl Responder {
    let conn=get_db_connection();
    
    let path_params=path.into_inner();
    let amount_io=path_params.0;
    let month=path_params.1;
    let year=path_params.2;
    
    let month_str:String;
    if month < 10{
        month_str=format!("0{month}");
    }else{
        month_str=format!("{month}");
    }

    let sql=format!("SELECT R.amount_io, R.category_id,C.name AS category_name ,SUM(R.amount) AS total FROM records R LEFT JOIN categories C ON R.category_id=C.id WHERE R.record_date LIKE '{year}-{month_str}-%' AND R.amount_io='{amount_io}' AND R.is_deleted=0 GROUP BY R.category_id ORDER BY R.amount DESC");
    let result_vec=execute_query_and_parse_balance(&conn, &sql);

    HttpResponse::Ok().json(json!({"success": true,"records": result_vec}))
}
/// Get all the amounts by amout_in (in/out) With categories by month and year

#[get("/balances/total_amount_of_category_name/{category}/{month}/{year}")]
pub async fn find_amouts_io_by_category_and_month(path: web::Path<(String,u8,u16)>) -> impl Responder {
    let conn=get_db_connection();
    
    let path_params=path.into_inner();
    let category=path_params.0;
    let month=path_params.1;
    let year=path_params.2;
    
    let month_str:String;
    if month < 10{
        month_str=format!("0{month}");
    }else{
        month_str=format!("{month}");
    }

    let sql=format!("SELECT R.amount_io, R.category_id,C.name AS category_name ,SUM(R.amount) AS total FROM records R LEFT JOIN categories C ON R.category_id=C.id WHERE R.record_date LIKE '{year}-{month_str}-%'  AND C.name LIKE '{category}' AND R.is_deleted=0 GROUP BY R.category_id ORDER BY R.amount DESC");
    let result_vec=execute_query_and_parse_balance(&conn, &sql);

    HttpResponse::Ok().json(json!({"success": true,"records": result_vec}))
}

#[get("/balances/sum_amount/{amount_io}/{month}/{year}")]
pub async fn sum_amouts_io_by_month(path: web::Path<(String,u8,u16)>) -> impl Responder {
    let conn=get_db_connection();
    
    let path_params=path.into_inner();
    let amount_io=path_params.0;
    let month=path_params.1;
    let year=path_params.2;
    
    let month_str:String;
    if month < 10{
        month_str=format!("0{month}");
    }else{
        month_str=format!("{month}");
    }

    let sql=format!("SELECT SUM(R.amount) AS total FROM records R WHERE R.record_date LIKE '{year}-{month_str}-%' AND R.amount_io='{amount_io}' AND R.is_deleted=0 LIMIT 1");
    let result_vec=execute_query_and_parse_amount_sum(&conn, &sql);

    HttpResponse::Ok().json(json!({"success": true,"records": result_vec}))
}

// Hacer las peticiones para:

// get_amouts_categories_by_month


fn execute_query_and_parse_balance(conn: &Connection, sql:&str) -> Vec<Balance>{
    let mut stmt = conn.prepare(&sql).unwrap();
    let elements_iter = stmt.query_map([], |row| {

        Ok(Balance {
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

fn execute_query_and_parse_amount_sum(conn: &Connection, sql:&str) -> Vec<BalanceSum>{
    let mut stmt = conn.prepare(&sql).unwrap();
    let elements_iter = stmt.query_map([], |row| {

        Ok(BalanceSum {
            total:row.get(0)?
        })
    }).unwrap();

    let mut result_vec=vec![];

    for element in elements_iter {
        result_vec.push(element.unwrap());
    }
    return result_vec;
}