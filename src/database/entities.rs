use super::schema::*;
use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use meta::{GetAll, GetById};
use serde::{Deserialize, Serialize};

#[derive(
  GetById, GetAll, Identifiable, Selectable, Queryable, Serialize, Deserialize, Debug, PartialEq,
)]
#[diesel(table_name = authors)]
#[meta::backend(connection = PgConnection)]
pub struct Author {
  pub id: i64,
  pub name: String,
  pub display_name: String,
  pub created: NaiveDateTime,
}

#[derive(
  GetById, GetAll, Identifiable, Selectable, Queryable, Serialize, Deserialize, Debug, PartialEq,
)]
#[diesel(table_name = books)]
#[meta::backend(connection = PgConnection)]
pub struct Book {
  pub id: i64,
  pub name: String,
  pub description: Option<String>,
}

#[derive(
  GetById,
  GetAll,
  Identifiable,
  Selectable,
  Queryable,
  Associations,
  Serialize,
  Deserialize,
  Debug,
  PartialEq,
)]
#[diesel(belongs_to(Book))]
#[diesel(belongs_to(Author))]
#[diesel(table_name = authors_of_books)]
#[meta::backend(connection = PgConnection)]
pub struct AuthorOfBook {
  pub id: i64,
  pub author_id: i64,
  pub book_id: i64,
}
