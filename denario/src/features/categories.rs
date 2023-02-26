use actix_web::{get,post,patch,delete, HttpResponse, Responder, web};
use serde::Deserialize;
use serde_json::json;
use rusqlite::{ Connection, NO_PARAMS, Statement, Error };
use dotenv::dotenv;
use std::env;

#[derive(Debug,serde::Serialize)]
struct Category{
    id:i32,
    name:String,
    created_at:String,
    updated_at:String,
    is_deleted:bool,
}

#[derive(Debug,Deserialize)]
struct DtoCategoryCreate{
    name:String, 
}
#[derive(Debug,Deserialize)]
struct DtoCategoryUpdate{
    name:String,
}

#[get("/categories")]
async fn find_all_categories() -> impl Responder {
    // DB CONN
    let db=env::var("DB_NAME").unwrap();
    let conn = Connection::open(db).unwrap();
    
    
    let mut stmt = conn.prepare("SELECT * FROM categories WHERE is_deleted=0").unwrap();
    let category_iter = stmt.query_map([], |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
            updated_at: row.get(3)?,
            is_deleted: row.get(4)?,
        })
    }).unwrap();

    let mut result_vec=vec![];

    for category in category_iter {
        result_vec.push(category.unwrap());
    }

    HttpResponse::Ok().json(json!({"status": "200","categories": result_vec}))
}

#[get("/categories/{id}")]
async fn find_one_category(path: web::Path<(u32,)>) -> impl Responder {
    // DB CONN
    let db=env::var("DB_NAME").unwrap();
    let conn = Connection::open(db).unwrap();
    
    let id=path.into_inner().0;
    let sql=format!("SELECT * FROM categories WHERE id= {id} AND is_deleted=1");
    let mut stmt = conn.prepare(&sql).unwrap();
    let category_iter = stmt.query_map([], |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
            updated_at: row.get(3)?,
            is_deleted: row.get(4)?,
        })
    }).unwrap();

    let mut result_vec=vec![];

    for category in category_iter {
        result_vec.push(category.unwrap());
    }

    HttpResponse::Ok().json(json!({"status": "success","categories": result_vec}))
}

#[post("/categories")]
async fn create_category(data: web::Json<DtoCategoryCreate>) -> impl Responder {
    // DB CONN
    let db=env::var("DB_NAME").unwrap();
    let conn = Connection::open(db).unwrap();
    
    // insert
    let name=data.into_inner().name;
    // println!("{:?}",name);
    let sql="INSERT INTO categories (name,created_at,updated_at,is_deleted) VALUES (?1,datetime('now'),datetime('now'),false)";
    conn.execute(&sql,&[&name.to_string()]);
    
    // get last inserted category
    let last_id= conn.last_insert_rowid();
    let sql=format!("SELECT * FROM categories WHERE id={last_id}");
    let mut stmt = conn.prepare(&sql).unwrap();
    let category_iter = stmt.query_map([], |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
            updated_at: row.get(3)?,
            is_deleted: row.get(4)?,
        })
    }).unwrap();

    let mut result_vec=vec![];

    for category in category_iter {
        result_vec.push(category.unwrap());
    }

    // match result {
    //     Err(e)=>{ HttpResponse::BadRequest().json(json!({"code":500,"success":false})) },
    //     Ok(cat)=>{ HttpResponse::Ok().json(json!({"success": true,"name":name,"categories": result_vec})) }
    // }
    HttpResponse::Ok().json(json!({"success": true,"categories": result_vec}))
}

#[patch("/categories/{id}")]
async fn update_category(path: web::Path<(u32,)>, data: web::Json<DtoCategoryUpdate>) -> impl Responder {
    // DB CONN
    let db=env::var("DB_NAME").unwrap();
    let conn = Connection::open(db).unwrap();

    let id=path.into_inner().0;
    let name=data.into_inner().name;

    // update
    let sql=format!("UPDATE categories SET name=?1, updated_at=datetime('now') WHERE id={}",id);
    conn.execute(&sql,&[&name.to_string()]);

    // select updated category
    let sql=format!("SELECT * FROM categories WHERE id={id}");
    let mut stmt = conn.prepare(&sql).unwrap();
    let category_iter = stmt.query_map([], |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
            updated_at: row.get(3)?,
            is_deleted: row.get(4)?,
        })
    }).unwrap();

    let mut result_vec=vec![];

    for category in category_iter {
        result_vec.push(category.unwrap());
    }

    HttpResponse::Ok().json(json!({"status": "success","categories": result_vec}))
}
#[delete("/categories/{id}")]
async fn delete_category(path: web::Path<(u32,)>) -> impl Responder {
    // DB CONN
    let db=env::var("DB_NAME").unwrap();
    let conn = Connection::open(db).unwrap();

    let id=path.into_inner().0;

    // update
    let sql=format!("UPDATE categories SET is_deleted=1, updated_at=datetime('now') WHERE id={}",id);
    conn.execute(&sql,NO_PARAMS);

    HttpResponse::Ok().json(json!({"status": "success","deleted": id}   ))
}