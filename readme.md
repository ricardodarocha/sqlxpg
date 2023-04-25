# Template Rust Api + Postgres

I would like to share with you a tiny preconfigured template to create amazing apis with Rust, using routes, json and so on.  

```
api_folder\
|- main.rs
|- routes.rs
|- models.rs
```

## Installation

First check you have already Rust installed.  
Clone this repositorio [Git](https://github.com/ricardodarocha/sqlxpg.git)  
Run `cargo check cargo run`

```Shell
git clone https://github.com/ricardodarocha/sqlxpg.git
cd sqlxpg 
cargo run
```

Navigate to
```
GET localhost:8089/user
```


> **Note:** [Check the compatible type to Rust + Postgre](https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html)

# Summary

This server is preconfigured in port 8089  
This server has the following routes
/ping  
/pong  
/user  
/post_user  

https://github.com/ricardodarocha/sqlxpg/blob/master/src/api/routes.rs  

The Postgre database connection is seted in https://github.com/ricardodarocha/sqlxpg/blob/master/src/main.rs  

# Models

Models represent the struct of sql, tables em views
https://github.com/ricardodarocha/sqlxpg/blob/master/src/model/usermodel.rs 

The models implemments FromRow do serialization



