use actix_web::{HttpServer, web, App, middleware::Logger};
use config::ConfigError;
use deadpool_diesel::{InteractError, Runtime};
use diesel::{PgConnection, pg::Pg};
use rust_backend::database::{Database, MIGRATIONS};
use serde::Deserialize;
use rust_backend::api;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub listen: String,
  pub workers: usize,
  pub database_url: String,
}

impl Config {
  pub fn from_env() -> Result<Self, ConfigError> {
    config::Config::builder()
      .add_source(config::Environment::default())
      .build()
      .unwrap()
      .try_deserialize()
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenvy::dotenv().expect("Could not load '.env' file.");
  let config = Config::from_env().expect("Could not load config file from environment.");
  let database: Database<PgConnection, Pg> = Database::<PgConnection, Pg>::new(&config.database_url, Runtime::Tokio1);
  let migration_result: Result<(), InteractError> = database.run_pending_migrations(MIGRATIONS).await;

  match migration_result {
    Ok(_) => {}
    Err(err) => log::error!("Error executing the migrations: {:?}", err),
  }

  let server = HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(database.clone()))
      .wrap(Logger::new("%a %{User-Agent}i"))
      .service(api::get_books)
      .service(api::get_authors)
      .service(api::healthcheck)
  })
  .workers(config.workers)
  .bind(&config.listen)?
  .run();

  server.await
}
