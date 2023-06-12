pub mod entities;
pub mod schema;

use std::marker::PhantomData;

use deadpool_diesel::{InteractError, Manager, Pool, Runtime};
use diesel::{backend::Backend, Connection};
use diesel::{PgConnection, pg::Pg};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct Database<C, B>
where
  C: Connection + MigrationHarness<B> + 'static,
  B: Backend,
{
  pool: Pool<Manager<C>>,
  _marker: PhantomData<B>,
}

impl<C, B> Database<C, B>
where
  C: Connection + MigrationHarness<B>,
  B: Backend,
{
  pub fn new(url: impl Into<String>, runtime: Runtime) -> Self {
    Self {
      pool: Pool::builder(Manager::<C>::new(&url.into(), runtime))
        .max_size(8)
        .build()
        .expect("Could not create connection pool for database"),
      _marker: PhantomData,
    }
  }

  pub async fn interact<F, R>(&self, f: F) -> Result<R, InteractError>
  where
    F: FnOnce(&mut C) -> R + Send + 'static,
    R: Send + 'static,
  {
    self
      .pool
      .get()
      .await
      .expect("Could not get a connection")
      .interact(f)
      .await
  }

  pub async fn run_pending_migrations(&self, migrations: EmbeddedMigrations) -> Result<(), InteractError> {
    self
      .interact(
        |conn| match MigrationHarness::<B>::run_pending_migrations(conn, migrations) {
          Ok(version) => log::debug!("Migrated to version {:?}", version),
          Err(err) => log::error!("Error running migrations: {:?}", err),
        },
      )
      .await
  }
}

impl<C, B> Clone for Database<C, B>
where
  C: Connection + MigrationHarness<B>,
  B: Backend,
{
  fn clone(&self) -> Self {
    Self {
      pool: self.pool.clone(),
      _marker: self._marker.clone(),
    }
  }
}

pub type DatabasePool = Database<PgConnection, Pg>;

#[macro_export]
macro_rules! impl_get_by_id {
  ($table:ident,$m:ident,$pool:ident,$id:ident) => {{
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;
    
    let conn = $pool.get()?;
    let res = $table::table
      .find($id)
      .first::<$m>(conn)
    Ok(res) as Result<$m, Error>
  }};
}
