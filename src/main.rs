#[macro_use]
extern crate juniper;

extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
mod examples;

//Actix imports will be used later for APIs
use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};


fn establish_connection() -> PgConnection {
    // load env file from root or parent
    dotenv().ok();
    // Read connection string from env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to DB {}", database_url))
}

fn borrower<'r>(a :&str, b:&'r str)->&'r str{
    // lifetime of b is named and its return type is mentioned so rust knows which lifetime to use
    // otherwise it wont know which lifetime to return if we skip 'r (named lifetime)
    b
}

fn main() {
    let connection = establish_connection();
    // example to demonstrate named lifetime
    borrower("a", "b");
    examples::getUsersList(&connection);
    examples::getTodosList(&connection);
    examples::joinUserTodo(&connection);
    examples::joinWithSelectedColumns(&connection);
}
