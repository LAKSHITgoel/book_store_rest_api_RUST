#![feature(plugin, const_fn, proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate r2d2;
extern crate dotenv;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use crate::diesel::Connection;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use rocket_codegen::routes;
use routes::*;
use std::env;

mod db;
mod models;
mod routes;
mod schema;
mod static_files;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Set your Db URL");

    let pool = db::init_pool(db_url);
    rocket::ignite()
        .manage(pool)
        .mount(
            "/api/v1/",
            routes![
                test,
                index,
                new,
                get_book_by_id,
                update_book_by_id,
                delete_book_by_id,
                get_all_books_by_author
            ],
        )
        .mount("/", routes![static_files::all, static_files::index])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
