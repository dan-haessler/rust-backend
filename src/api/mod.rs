use crate::database::{
  entities::{Author, Book},
  DatabasePool,
};
use actix_web::{get, web, HttpResponse, Responder};
use meta::{rest_get_all, rest_get_by_id, GetAll, GetById};

#[get("/health")]
pub async fn healthcheck() -> impl Responder {
  "Healthy!"
}

#[rest_get_all(Book)]
#[get("/books")]
pub async fn get_books(pool: web::Data<DatabasePool>) -> impl Responder {}

#[rest_get_by_id(Book)]
#[get("/books/{id}")]
pub async fn get_book_by_id(path: web::Path<i64>, pool: web::Data<DatabasePool>) -> impl Responder {
}

#[rest_get_all(Author)]
#[get("/authors")]
pub async fn get_authors(pool: web::Data<DatabasePool>) -> impl Responder {}

#[rest_get_by_id(Author)]
#[get("/authors/{id}")]
pub async fn get_author_by_id(
  path: web::Path<i64>,
  pool: web::Data<DatabasePool>,
) -> impl Responder {
}

pub mod error;
