pub trait GetById<T, I> {
  fn get_by_id(connection: &mut diesel::PgConnection, entity_id: I) -> Result<T, diesel::result::Error>;
}

pub trait GetAll<T> {
  fn get_all(connection: &mut diesel::PgConnection) -> Result<Vec<T>, diesel::result::Error>;
}

pub trait Insert<T> {
  fn insert(connection: &mut diesel::PgConnection, entity: &T) -> Result<usize, diesel::result::Error>;
}

pub trait Update<T> {
  fn update(connection: &mut diesel::PgConnection, entity: &T) -> Result<usize, diesel::result::Error>;
}

