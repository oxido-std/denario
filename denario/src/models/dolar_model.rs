use serde::Deserialize;

#[derive(Debug,serde::Serialize)]
pub struct Dolar{
   pub id:i64,
   pub name:String,
   pub amount:f32,
   pub source:String,
   pub created_at:String,
   pub is_deleted:bool,
}

pub trait SQLDolar {
    fn get_query_insert() -> String{
        format!("INSERT INTO dolars (name,amount,source,created_at,is_deleted) VALUES (?1,?2,?3,datetime('now'),false)")
    }

    fn get_query_delete(id:u32) -> String{
        format!("UPDATE dolars SET is_deleted=1, updated_at=datetime('now') WHERE id={}",id)
    }
}

impl SQLDolar for Dolar{

}

#[derive(Debug,Deserialize)]
pub struct DtoDolar{
    pub name:String,
    pub amount:f32,
    pub source:String,
}