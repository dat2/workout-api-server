use diesel::pg::PgConnection;
use errors;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::{Request, State, Outcome};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use std::env;
use std::ops::Deref;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn pool() -> errors::Result<Pool> {
  let config = r2d2::Config::default();
  let manager = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL")?);
  r2d2::Pool::new(config, manager).map_err(|e| e.into())
}

pub struct DbConn {
  pub conn: r2d2::PooledConnection<ConnectionManager<PgConnection>>,
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
    let pool = request.guard::<State<Pool>>()?;
    match pool.get() {
      Ok(conn) => Outcome::Success(DbConn { conn: conn }),
      Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
    }
  }
}

impl Deref for DbConn {
  type Target = PgConnection;

  fn deref(&self) -> &Self::Target {
    &self.conn
  }
}
