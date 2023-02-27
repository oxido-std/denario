use serde::Deserialize;

#[derive(Debug,serde::Serialize)]
pub struct Record{
    pub id:i64,
    pub name:String,
    pub amount:f32,
    pub amount_io:String, // in / out
    pub comment:String,
    pub record_date:String,
    pub category_id:i64,
    pub created_at:String,
    pub updated_at:String,
    pub is_deleted:bool,
}
pub trait SQLRecord {
    fn get_query_insert() -> String{
        format!("INSERT INTO records (name,amount,amount_io,comment,record_date,category_id,created_at,updated_at,is_deleted) VALUES (?1,?2,?3,?4,?5,?6,datetime('now'),datetime('now'),false)")
    }

    fn get_query_update(id:u32) -> String{
        format!("UPDATE records SET name=?1,amount=?2,amount_io=?3,comment=?4,record_date=?5,category_id=?6,updated_at=datetime('now') WHERE id={}",id)
    }

    fn get_query_delete(id:u32) -> String{
        format!("UPDATE records SET is_deleted=1, updated_at=datetime('now') WHERE id={}",id)
    }
}

impl SQLRecord for Record{

}

#[derive(Debug,serde::Serialize)]
pub struct Record2Categories{
    pub amount_io:String, // in / out
    pub category_id:i64,
    pub category_name:String,
    pub total:f32,
}

#[derive(Debug,Deserialize)]
pub struct DtoRecord{
    pub name:String,
    pub amount:f32,
    pub amount_io:String,
    pub comment:String,
    pub record_date:String,
    pub category_id:i64,
}

#[derive(Debug,Deserialize)]
pub struct FiltersRecord{
    pub limit:u16,
    pub skip:u16,
    pub orderby_column:String,
    pub orderby:String,
}