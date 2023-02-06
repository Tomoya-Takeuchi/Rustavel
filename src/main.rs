use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::{env,process};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use sqlx::Row;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    dotenv().ok();
    let database_url=match env::var("DATABASE_URL"){
        Ok(ok)=>ok,
        Err(err)=>{
            eprint!("Error: std::env said, {}\n",err);
            process::exit(1);
        }
    };
    let pool=match MySqlPool::connect(&database_url).await{
        Ok(ok)=>ok,
        Err(err)=>{
            eprint!("Error: sqlx said, {}\n",err);
            process::exit(1);
        }
    };

    let rows = match sqlx::query("SELECT * FROM employee")
    .fetch_all(&pool)
    .await{
        Ok(ok)=>ok,
        Err(err)=>{
            eprint!("Error: sqlx said, {}\n",err);
            process::exit(1);
        }
    };
    println!("{:?}", rows);
    let str_result = rows
		.iter()
		.map(|r| format!("{} - {}", r.get::<i64, _>("id"), r.get::<String, _>("name")))
		.collect::<Vec<String>>()
		.join(", ");
	println!("\n== select employee with rows:\n{}", str_result);

    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("app", 8080))?
    .run()
    .await
}