#[derive(Debug,serde::Serialize)]
pub struct RecordAndCategory{
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
    pub is_mutable:bool,
    pub category_name:String,
}

#[derive(Debug,serde::Serialize)]
pub struct Balance{
    pub amount_io:String,
    pub category_id:i64,
    pub category_name:String,
    pub total:f32,
}

#[derive(Debug,serde::Serialize)]
pub struct BalanceSum{
    pub total:f32,
}