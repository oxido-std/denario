use serde::Deserialize;

#[derive(Debug,serde::Serialize)]
pub struct Category{
    pub id:i64,
    pub name:String,
    pub created_at:String,
    pub updated_at:String,
    pub is_deleted:bool,
}

#[derive(Debug,Deserialize)]
pub struct DtoCategory{
    pub name:String, 
}