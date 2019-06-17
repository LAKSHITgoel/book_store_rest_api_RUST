#![feature(plugin, const_fn, proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use diesel::pg::PgConnection;
use crate::diesel::Connection;
use dotenv::dotenv;
use std::env;
use rocket_codegen::routes;

mod models;
mod schema;
mod db;
mod static_files;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Set your Db URL");

    let pool = db::init_pool(db_url);
    rocket::ignite()
        .manage(pool)
        .mount("/",routes![static_files::all, static_files::index])
}



fn main() {
    rocket().launch();
}
