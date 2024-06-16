use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::models::Book;
use std::sync::Mutex;

pub type Db = Mutex<Vec<Book>>;

// Get all books
pub async fn get_books(db: web::Data<Db>) -> HttpResponse {
    let books = db.lock().unwrap();
    HttpResponse::Ok().json(&*books)
}

// Get a single book by ID
pub async fn get_book(path: web::Path<Uuid>, db: web::Data<Db>) -> HttpResponse {
    let books = db.lock().unwrap();
    let book = books.iter().find(|&book| book.id == *path);
    if let Some(book) = book {
        HttpResponse::Ok().json(book)
    } else {
        HttpResponse::NotFound().finish()
    }
}

// Create a new book
pub async fn create_book(new_book: web::Json<Book>, db: web::Data<Db>) -> HttpResponse {
    let mut books = db.lock().unwrap();
    let mut book = new_book.into_inner();
    book.id = Uuid::new_v4();
    books.push(book);
    HttpResponse::Created().finish()
}

// Update an existing book
pub async fn update_book(path: web::Path<Uuid>, updated_book: web::Json<Book>, db: web::Data<Db>) -> HttpResponse {
    let mut books = db.lock().unwrap();
    let book = books.iter_mut().find(|book| book.id == *path);
    if let Some(book) = book {
        book.title = updated_book.title.clone();
        book.author = updated_book.author.clone();
        HttpResponse::Ok().json(book)
    } else {
        HttpResponse::NotFound().finish()
    }
}

// Delete a book
pub async fn delete_book(path: web::Path<Uuid>, db: web::Data<Db>) -> HttpResponse {
    let mut books = db.lock().unwrap();
    if books.iter().any(|book| book.id == *path) {
        books.retain(|book| book.id != *path);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
