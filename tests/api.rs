use actix_http::Request;
use actix_service::Service;
use actix_web::dev::{HttpServiceFactory, ServiceResponse};
use actix_web::{test, App};

use actix_web::web;
use actix_web::Error;
use deadpool_diesel::{InteractError, Runtime};
use diesel::{pg::Pg, PgConnection};
use rust_backend::{
  database::{Database, MIGRATIONS},
  Config,
};

pub async fn init() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
  dotenvy::dotenv().expect("Could not load '.env' file.");
  let config = Config::from_env().expect("Could not load config file from environment.");
  let database: Database<PgConnection, Pg> =
    Database::<PgConnection, Pg>::new(&config.database_url, Runtime::Tokio1);
  let migration_result: Result<(), InteractError> =
    database.run_pending_migrations(MIGRATIONS).await;

  match migration_result {
    Ok(_) => {}
    Err(err) => log::error!("Error executing the migrations: {:?}", err),
  }

  test::init_service(
    App::new()
      .app_data(web::Data::new(database.clone()))
      .service(rust_backend::api::get_books)
      .service(rust_backend::api::get_book_by_id)
      .service(rust_backend::api::get_authors)
      .service(rust_backend::api::get_author_by_id)
      .service(rust_backend::api::healthcheck),
  )
  .await
}

// find out how to modularise the tests, one app per integration test does not work. The initialisation stops indefinitely.
#[actix_web::test]
pub async fn test_all() {
  let app = init().await;

  let req = test::TestRequest::get().uri("/authors").to_request();
  let resp = test::call_service(&app, req).await;
  assert!(resp.status().is_success());

  let req = test::TestRequest::get().uri("/authors/1").to_request();
  let resp = test::call_service(&app, req).await;
  assert!(resp.status().is_success());

  let req = test::TestRequest::get().uri("/books").to_request();
  let resp = test::call_service(&app, req).await;
  assert!(resp.status().is_success());

  let req = test::TestRequest::get().uri("/books/1").to_request();
  let resp = test::call_service(&app, req).await;
  assert!(resp.status().is_success());
}
