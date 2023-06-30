use diesel::prelude::*;
use chrono::{NaiveDateTime};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category{
    pub id:i32,
    pub name:String,
    pub created_at:NaiveDateTime,
    pub updated_at:NaiveDateTime,
    pub is_deleted:bool,
}