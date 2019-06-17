use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::books;
use crate::schema::books::dsl::books as all_books;

#[derive(Serialize,Queryable,Debug,Clone)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published: bool,
}

#[derive(Serialize,Deserialize,Insertable)]
#[table_name = "books"]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub published: bool,
}

impl Book {
    pub fn get_book_by_id(id: i32, conn: &PgConnection) -> Vec<Book> {
        all_books
            .find(id)
            .load::<Book>(conn)
            .expect("Error loading book")
    }

    pub fn get_all_books(conn: &PgConnection) -> Vec<Book> {
        all_books
            .order(books::id.desc())
            .load::<Book>(conn)
            .expect("Error loading books")
    }

    pub fn update_book_by_id(id: i32, conn: &PgConnection, book: NewBook) -> bool {
        use crate::schema::books::dsl::{author as a, published as p, title as t};
        let NewBook {
            title,
            author,
            published,
        } = book;
        diesel::update(all_books.find(id))
            .set((a.eq(author), t.eq(title), p.eq(published)))
            .get_result::<Book>(conn)
            .is_ok()
    }

    pub fn insert_book( book: NewBook, conn: &PgConnection) -> bool {
        diesel::insert_into(books::table)
            .values(&book)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_book_by_id(id: i32, conn: &PgConnection) -> bool {
        if Book::get_book_by_id(id, conn).is_empty() {
            return false;
        } else {
            diesel::delete(all_books.find(id)).execute(conn).is_ok()
        }
    }

    pub fn get_all_books_by_author(author: String, conn: &PgConnection) -> Vec<Book> {
        all_books
            .filter(books::author.eq(author))
            .load::<Book>(conn)
            .expect("Error loading books by author")
    }
}
