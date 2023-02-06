use actix_web::{web, App, HttpServer};
use std::{env,process};
use dotenv::dotenv;
use sqlx::pool::Pool;
use sqlx::mysql::*;
mod handlers;
mod repositories;

use crate::handlers::*;
use crate::repositories::{EmployeeRepository, EmployeeRepositoryForDB};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let context = match RequestContext::new().await {
        Ok(ok)=> ok,
        Err(err) => {
            eprint!("Error: std::env said, {}\n",err);
            process::exit(1);
        }
    };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(context.clone()))
            .service(list_employee)
            .service(create_employee)
    })
    .bind(("app", 8080))?
    .run()
    .await
}

#[derive(Clone)]
pub struct RequestContext {
    pool: Pool<MySql>,
}

impl RequestContext {
    pub async fn new() -> anyhow::Result<RequestContext> {
        dotenv().ok();
        let database_url=match env::var("DATABASE_URL"){
            Ok(ok)=>ok,
            Err(err)=>{
                eprint!("Error: std::env said, {}\n",err);
                process::exit(1);
            }
        };
        let pool= match MySqlPool::connect(&database_url).await{
            Ok(ok)=>ok,
            Err(err)=>{
                eprint!("Error: sqlx said, {}\n",err);
                process::exit(1);
            }
        };

        Ok(RequestContext { pool })
    }

    pub fn employee_repository(&self) -> impl EmployeeRepository {
        EmployeeRepositoryForDB::new(self.pool.to_owned())
    }
}