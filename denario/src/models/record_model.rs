use serde::Deserialize;

#[derive(Debug,serde::Serialize)]
pub struct Record{
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
}

#[derive(Debug,serde::Serialize)]
/// This is just for SELECT query. You will get the record element JOIN the category name.
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
pub trait SQLRecord {
    fn get_query_insert() -> String{
        format!("INSERT INTO records (name,amount,amount_io,comment,record_date,category_id,created_at,updated_at,is_deleted,is_mutable) VALUES (?1,?2,?3,?4,?5,?6,datetime('now'),datetime('now'),false,true)")
    }
    
    /// Esta función cambia la fecha del registro para poder utilizarla con las funcions SQL de
    /// 
    /// date([FECHA],'+[CANT] [DAY /MONTH / YEARS])

    fn get_query_insert_future(time_plus:u8) -> String{
        format!("INSERT INTO records (name,amount,amount_io,comment,record_date,category_id,created_at,updated_at,is_deleted,is_mutable) VALUES (?1,?2,?3,?4,date(?5,'+{} month'),?6,datetime('now'),datetime('now'),false,false)",time_plus)
    }

    fn get_query_update(id:u32) -> String{
        format!("UPDATE records SET name=?1,amount=?2,amount_io=?3,comment=?4,record_date=?5,category_id=?6,updated_at=datetime('now') WHERE id={} AND is_mutable=1",id)
    }

    fn get_query_delete(id:u32) -> String{
        format!("UPDATE records SET is_deleted=1, updated_at=datetime('now') WHERE id={} AND is_mutable=1",id)
    }
}

impl SQLRecord for Record{

}

#[derive(Debug,serde::Serialize)]
pub struct Record2Categories{
    pub amount_io:String, // in / out
    pub category_id:i64,
    pub category_name:String,
    pub total:f32,
}

#[derive(Debug,Deserialize)]
pub struct DtoRecord{
    pub name:String,
    pub amount:f32,
    pub amount_io:String,
    pub comment:String,
    pub record_date:String,
    pub category_id:i64,
    pub is_mutable:bool,
}

#[derive(Debug,Deserialize)]
pub struct FiltersRecord{
    pub limit:u16,
    pub skip:u16,
    pub orderby_column:String,
    pub orderby:String,
}