# Template Rust Api + Postgres

Template para criar uma nova api com Rust usando Actix  
Esta versão atualizada de newapi/ traz suporte para newapi_json com Postgres


```
api_folder\
|- main.rs
|- routes.rs
|- models.rs
```

A maneira mais rápida criando este repositório é instalar o **Rust** e clonar o repositório

```Shell
git clone https://github.com/ricardodarocha/newapi_json.git
cd sqlxpg 
cargo run
```

navegue
```
GET localhost:8089/user
```


> **Note:** [Confira os tipos de dados disponíveis para Rust + Postgre](https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html)
