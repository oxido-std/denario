use diesel::prelude::*;
use chrono::{NaiveDateTime,NaiveDate};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::records)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Record {
    pub id:i32,
    pub name:String,
    pub amount:f32,
    pub amount_io:String, // in / out
    pub comment:String,
    pub record_date:NaiveDate,
    pub category_id:i32,
    pub created_at:NaiveDateTime,
    pub updated_at:NaiveDateTime,
    pub is_deleted:bool,
    pub is_mutable:bool,
}

// #[derive(Queryable, Selectable)]
// #[diesel(table_name = crate::schema::posts)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct RecordAndCategory{
//     pub id:i64,
//     pub name:String,
//     pub amount:f32,
//     pub amount_io:String, // in / out
//     pub comment:String,
//     pub record_date:String,
//     pub category_id:i64,
//     pub created_at:String,
//     pub updated_at:String,
//     pub is_deleted:bool,
//     pub is_mutable:bool,
//     pub category_name:String,
// }