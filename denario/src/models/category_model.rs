use serde::Deserialize;

#[derive(Debug,serde::Serialize)]
pub struct Category{
    pub id:i64,
    pub name:String,
    pub created_at:String,
    pub updated_at:String,
    pub is_deleted:bool,
}

pub trait SQLCategory {
    fn get_query_insert() -> String{
        format!("INSERT INTO categories (name,created_at,updated_at,is_deleted) VALUES (?1,datetime('now'),datetime('now'),false)")
    }

    fn get_query_update(id:u32) -> String{
        format!("UPDATE categories SET name=?1, updated_at=datetime('now') WHERE id={}",id)
    }

    fn get_query_delete(id:u32) -> String{
        format!("UPDATE categories SET is_deleted=1, updated_at=datetime('now') WHERE id={}",id)
    }
}

impl SQLCategory for Category{

}

#[derive(Debug,Deserialize)]
pub struct DtoCategory{
    pub name:String, 
}