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