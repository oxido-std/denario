use actix_web::{Responder, HttpResponse, get};

use diesel::PgConnection;
use dotenv::dotenv;
use std::{env, fs};

use crate::db_conn::establish_connection;

pub struct AppSetup {
    pub db_conn: PgConnection,
    pub app_setup_eneabled: bool,

}

impl AppSetup {
    pub fn new() -> Self {
        dotenv().ok();
        let db_conn = establish_connection();
        let app_setup_eneabled = env::var("APP_SETUP_ENABLED").expect("APP_SETUP must be set") == "true";
        return Self { db_conn, app_setup_eneabled }
    }
    
    pub fn main(self) -> impl Responder {
        let mut response=format!("App Setup is disabled");
        
        if self.app_setup_eneabled {
            self.create_folders();
            self.plaint_seeds();

            response=format!("App Setup is enabled.\n Now running...");
        }
        HttpResponse::Ok().body(response)
    }
    
    
    fn mk_dir_in_current_folder(&self,new_dir:String){
        let mut path = env::current_dir().unwrap();
        path.push(new_dir);
        if !path.exists() {
            fs::create_dir_all(path).unwrap();
        }
    }
    
    fn create_file_in_current_folder(&self,new_file:String){
        let mut path = env::current_dir().unwrap();
        path.push(new_file);
        if !path.exists() {
            fs::write(path, "").unwrap();
        }
    }

    pub fn create_folders(&self){
        self.mk_dir_in_current_folder("public".to_string());
        self.mk_dir_in_current_folder("assets".to_string());
        self.mk_dir_in_current_folder("uploads".to_string());
        self.mk_dir_in_current_folder("seeds".to_string());
    }

    pub fn plaint_seeds(&self){        // seeds
        self.create_file_in_current_folder("seeds/users.json".to_string());
        self.create_file_in_current_folder("seeds/posts.json".to_string());
    }
}


#[get("/run_setup")]
pub async fn run_setup() -> impl Responder {
    let app_setup = AppSetup::new();
    app_setup.main()
}