#[macro_use]
extern crate diesel;

mod schema;
mod routes;
mod model;

use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;

use std::collections::HashMap;
use std::env;

fn main() {
    dotenv().ok();  // Loads the .env file

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set!");  // Reads DATABASE_URL value from .env file
    let manager = ConnectionManager::<PgConnection>::new(db_url);  // Connect to our database

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build the pool");
} 
