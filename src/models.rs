use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Bookstore {
    pub id: i32,
    pub nome: String,
    pub autor: String,
    pub editora: String,
    pub ano: String,
    pub codigobarras: String
}
