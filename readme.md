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

# Resumo

Este projeto inicia um servidor na porta 8089  
Disponibiliza as rotas  
/ping  
/pong  
/user  
/post_user  

https://github.com/ricardodarocha/sqlxpg/blob/master/src/api/routes.rs  

A conexão com banco Postgre é configurada https://github.com/ricardodarocha/sqlxpg/blob/master/src/main.rs  

# Modelagem

Os modelos que representam o banco de dados estão no arquivo 
https://github.com/ricardodarocha/sqlxpg/blob/master/src/model/usermodel.rs 

Os modelos para representar o select derivam de FromRow

# Modularização

O projeto utiliza módulos que são declarados no formato de pastas
Uma pasta para cada módulo, sendo que dentro de cada pasta possui um arquivo mod.rs
O arquivo mod.rs é consumido no fonte principal main.rs

