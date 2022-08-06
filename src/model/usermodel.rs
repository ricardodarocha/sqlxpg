use serde::{Serialize, Deserialize};
use sqlx::{FromRow};

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub nome: String,
    pub email: String,
    pub senha: String
}
#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub nome: String,
    pub email: String,
    pub senha: String
}
