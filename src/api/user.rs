use actix_web::{
        get, 
        post, 
        put, 
        delete, 
        web::Json,
        web::Data,
        http::{StatusCode},
        Responder, 
        HttpResponse};
use sqlx::postgres::{PgPool, PgRow}; 
use sqlx::{Row};

use crate::model::usermodel::User;        

#[post("/user/{id}")]
pub async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("create_user")
}

#[put("/user/{id}")]
pub async fn update_user() -> impl Responder {
    HttpResponse::Ok().body("update_user")
}

#[delete("/user")]
pub async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("delete_user")
}

