use actix_web::{middleware::Logger, web, App, HttpServer};
use deadpool_diesel::{Runtime};
use diesel::{pg::Pg, PgConnection};
use rust_backend::database::{Database, MIGRATIONS};
use rust_backend::{api, Config};
use simple_logger::SimpleLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  SimpleLogger::new().init().unwrap();

  match dotenvy::dotenv() {
    Ok(path) => log::info!("Loaded .env from {:?}", path),
    Err(err) => log::error!("Error loading .env {:?}", err),
  };
  let config = Config::from_env().expect("Could not load config file from environment.");
  let database: Database<PgConnection, Pg> =
    Database::<PgConnection, Pg>::new(&config, Runtime::Tokio1);

  match database.run_pending_migrations(MIGRATIONS).await {
    Ok(_) => {}
    Err(err) => log::error!("Error executing the migrations: {:?}", err),
  }

  let server = HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(database.clone()))
      .wrap(Logger::new("%a %{User-Agent}i"))
      .service(api::get_books)
      .service(api::get_book_by_id)
      .service(api::get_authors)
      .service(api::get_author_by_id)
      .service(api::healthcheck)
  })
  .workers(config.workers)
  .bind(&config.listen)?
  .run();

  server.await
}
