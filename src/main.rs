#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;

use diesel::pg::PgConnection;
use crate::diesel::Connection;
use dotenv::dotenv;
use std::env;

mod models;
mod schema;

fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Set your Db URL");
    let conn = PgConnection::establish(&db_url).unwrap();

    let book = models::NewBook {
        title: String::from("Gravity's rainbow"),
        author: String::from("Thomas Pynchon"),
        published: true,
    };

    if models::Book::insert_book(book, &conn) {
        println!("Success");
    } else {
        println!("Failed");
    }
}
