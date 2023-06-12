use actix_web::{HttpResponse, Responder, get, web};
use meta::{GetAll, GetById, rest_get_all, rest_get_by_id};
use crate::database::{DatabasePool, entities::{Book, Author}};

#[get("/health")]
pub async fn healthcheck() -> impl Responder {
    "Healthy!"
}

#[rest_get_all(Book)]
#[get("/books")]
pub async fn get_books(pool: web::Data<DatabasePool>) -> impl Responder {
}

#[rest_get_by_id(Book)]
#[get("/books/{id}")]
pub async fn get_book_by_id(path: web::Path<i64>, pool: web::Data<DatabasePool>) -> impl Responder {
}

#[rest_get_all(Author)]
#[get("/authors")]
pub async fn get_authors(pool: web::Data<DatabasePool>) -> impl Responder {
}

#[rest_get_by_id(Author)]
#[get("/authors/{id}")]
pub async fn get_author_by_id(path: web::Path<i64>, pool: web::Data<DatabasePool>) -> impl Responder {
}


pub mod error;
