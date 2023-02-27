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

#[derive(Debug,Deserialize)]
pub struct DtoDolar{
    pub name:String,
    pub amount:f32,
    pub source:String,
}