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

pub trait SQLCredit {
    fn get_query_insert(payments:&String) -> String{
        format!("INSERT INTO credits (name,comment,amount,payments,started_at,finish_at,category_id,created_at,updated_at,is_deleted) VALUES (?1,?2,?3,?4,?5,date(?5,'+{} month'),?6,datetime('now'),datetime('now'),false)",payments)
    }

    fn get_query_update(payments:&String,id:u32) -> String{
        format!("UPDATE credits SET name=?1,comment=?2,amount=?3,payments=?4,started_at=?5,finish_at=date(?5,'+{} month'),category_id=?6,updated_at=datetime('now') WHERE id={}",payments,id)
    }

    fn get_query_delete(id:u32) -> String{
        format!("UPDATE credits SET is_deleted=1, updated_at=datetime('now') WHERE id={}",id)
    }

    fn get_last_credit_id() -> String{
        format!("SELECT *,MAX(id) FROM credits LIMIT 1")
    }
}

impl SQLCredit for Credit{

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