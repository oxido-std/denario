use diesel::prelude::*;
use chrono::{NaiveDateTime,NaiveDate};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::credits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Credit{
    pub id:i32,
    pub name:String,
    pub comment:String,
    pub amount:f32,
    pub payments:i32,
    pub started_at:NaiveDate,
    pub finish_at:NaiveDate,
    pub category_id:i32,
    pub created_at:NaiveDateTime,
    pub updated_at:NaiveDateTime,
    pub is_deleted:bool,
}