use actix_web::{
    get, 
    post,
    put,
    delete,
    error::ResponseError,
    Error, 
    web,
    web::Path,
    web::Json,
    web::Data, 
    App, 
    HttpServer, 
    HttpResponse,
    Responder,
    http::{header::ContentType, StatusCode}
};

  use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
  use sqlx::{FromRow, Row};

use crate::model::usermodel::{User, NewUser};

#[get("/ping")]
pub async fn ping(pool: web::Data<PgPool>) -> impl Responder {
    let row = sqlx::query("select 1 as id")
        .fetch_one(pool.get_ref())
        .await
        .unwrap();
    let id: i32 = row.try_get("id").unwrap();
    format!("{:?}", id)
}

#[get("/pong")]
pub async fn pong(pool: web::Data<PgPool>) -> impl Responder {
    let row = sqlx::query("select 1 as id")
        .fetch_one(pool.get_ref())
        .await
        .unwrap();
    let one: i32 = row.try_get("id").unwrap();
    "{'test': 1}"
}

#[get("/user")]
pub async fn user(pool: web::Data<PgPool>) -> HttpResponse {
    let users = sqlx::query_as::<_, User>("select * from users")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(users) 
}

#[post("/user")]
pub async fn post_user(pool: web::Data<PgPool>, usr: web::Json<NewUser>) -> HttpResponse {
    let usercreated = sqlx::query_as::<_, User>(
        "insert into users
            (nome, email, senha) 
            VALUES ($1, $2, $3)
            returning id, nome, email, senha")
            .bind(usr.nome.to_owned())
            .bind(usr.email.to_owned())
            .bind(usr.senha.to_owned())
        .fetch_one(pool.get_ref())
        .await
        .unwrap();
        
      HttpResponse::Ok()
      .status(StatusCode::CREATED)
      .json(usercreated)
}