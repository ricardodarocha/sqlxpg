#![allow(unused)]

use actix_web::{
    App, 
    HttpServer, 
    Responder,
    middleware::Logger,
    http::{header::ContentType, StatusCode}
};
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
use std::env;

mod api;
mod model;

use model::models::Bookstore;
use std::net::TcpListener;
use api::routes::{
    ping,
    pong,
    user,
    post_user,
};


use api::user::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "debug");
    // std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    
    let pool = PgPool::connect("postgres://postgres:masterkey@localhost:5432/proto")
        .await
        .expect("⚠️ Não foi possível iniciar o Pool de conexão");

        HttpServer::new( move || {
        let logger = Logger::default();
        App::new()
        .data( pool.clone() )
        .wrap(logger)
        .service(ping)
        .service(pong)
        .service(user)
        .service(post_user)
    })
    .bind(("localhost", 8089))?
    .run()
    .await
}