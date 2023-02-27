use serde::Deserialize;

#[derive(Debug,serde::Serialize)]
pub struct Credit{
    pub id:i64,
    pub name:String,
    pub comment:String,
    pub amount:f32,
    pub payments:u16,
    pub started_at:String,
    pub finish_at:String,
    pub category_id:i64,
    pub created_at:String,
    pub updated_at:String,
    pub is_deleted:bool,
}

#[derive(Debug,Deserialize)]
pub struct DtoCredit{
    pub name:String,
    pub comment:String,
    pub amount:f32,
    pub payments:u16,
    pub started_at:String,
    pub category_id:i64,
}

#[derive(Debug,Deserialize)]
pub struct FiltersCredit{
    pub limit:u16,
    pub skip:u16,
    pub orderby_column:String,
    pub orderby:String,
}