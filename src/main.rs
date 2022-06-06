#[macro_use]
extern crate juniper;

extern crate dotenv;

use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

//Actix imports will be used later for APIs
use actix_cors::Cors;
use actix_web::{middleware::Logger,web::Data, App, HttpServer};

// import table object to use it in query
use server_test::schema::users::dsl::*;

use server_test::models::*;

fn establish_connection()->PgConnection{
    // load env file from root or parent
    dotenv().ok();
    // Read connection string from env
    let database_url=env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to DB {}", database_url))
}

fn main(){
    let connection = establish_connection(); 
    let results = users
        .limit(5)
        .load::<Users>(&connection)
        .expect("Error loading users");

    for usr in results {
        println!("{}", usr.username);
        println!("{}", usr.password);
    }
}
