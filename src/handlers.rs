use actix_web::{get, post, web, HttpResponse, Responder};
use std::{process};
use super::RequestContext;
use crate::repositories::{Employee, EmployeeRepository};

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