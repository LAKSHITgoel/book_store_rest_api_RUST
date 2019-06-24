#![feature(custom_attribute, proc_macro_hygiene, decl_macro)]

use super::db::Conn as DbConn;
use super::models::{Book, NewBook};
use rocket_contrib::json::Json;
use serde_json::Value;

#[get("/test", format = "application/json")]
pub fn test() -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": "Test Sucessfull"
    }))
}

#[get("/books", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let books = Book::get_all_books(&conn);
    Json(json!({
        "status" : 200,
        "result" : books
    }))
}

#[post("/books", format = "application/json", data = "<new_book>")]
pub fn new(new_book: Json<NewBook>, conn: DbConn) -> Json<Value> {
    Json(json!({
        "status"  :Book::insert_book(new_book.into_inner(),&conn),
        "result"  :Book::get_all_books(&conn).first()
    }))
}

#[get("/books/<id>", format = "application/json")]
pub fn get_book_by_id(id: i32, conn: DbConn) -> Json<Value> {
    let result = Book::get_book_by_id(id, &conn);
    let status = if result.is_empty() { 400 } else { 200 };
    Json(json!({
        "status" : status,
        "result" : result
    }))
}

#[put("/books/<id>", format = "application/json", data = "<updated_book>")]
pub fn update_book_by_id(id: i32, conn: DbConn, updated_book: Json<NewBook>) -> Json<Value> {
    let status = if Book::update_book_by_id(id.clone(), &conn, updated_book.into_inner()) {
        200
    } else {
        404
    };
    let result = Book::get_book_by_id(id.clone(), &conn);

    Json(json!({
        "status":status,
        "result":result
    }))
}

#[delete("/books/<id>", format = "application/json")]
pub fn delete_book_by_id(id: i32, conn: DbConn) -> Json<Value> {
    let status = if Book::delete_book_by_id(id, &conn) {
        200
    } else {
        404
    };

    Json(json!({
        "status":status,
        "result": null
    }))
}

#[get("/books/author/<author>", format = "application/json")]
pub fn get_all_books_by_author(author: String, conn: DbConn) -> Json<Value> {
    let result = Book::get_all_books_by_author(author, &conn);
    let status = if result.is_empty() { 400 } else { 200 };
    Json(json!({
        "status":status,
        "result":result
    }))
}

#[catch(404)]
pub fn not_found() -> Json<Value> {
    Json(json!({
        "status":"404",
        "result":"HOLY COW"
    }))
}