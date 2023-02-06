use actix_web::{get, post, web, HttpResponse, Responder};
use std::{process};
use super::RequestContext;
use crate::repositories::{Employee, EmployeeRepository};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/employee/list")]
pub async fn list_employee(data: web::Data<RequestContext>) -> impl Responder {
    let rows = match data.employee_repository().list().await {
        Ok(ok)=>ok,
        Err(err)=>{
            eprint!("Error: sqlx said, {}\n",err);
            process::exit(1);
        }
    };
    let str_result = rows
		.iter()
		.map(|r| format!("{} - {}", r.id.to_string(), r.name.to_string()))
		.collect::<Vec<String>>()
		.join(", ");
    HttpResponse::Ok().body(str_result)
}

#[post("/employee/create")]
pub async fn create_employee(req: web::Json<Employee>) -> impl Responder {
    let employee = Employee {
        id: req.id,
        name: req.name.to_string(),
    };
    HttpResponse::Ok().json(employee)
}