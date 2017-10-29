use errors;
use r2d2;
use r2d2_redis::RedisConnectionManager;
use redis::Connection;
use rocket::{Request, State, Outcome};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use std::default::Default;
use std::env;
use std::ops::Deref;

type Pool = r2d2::Pool<RedisConnectionManager>;

pub fn pool() -> errors::Result<Pool> {
  let config = Default::default();
  let redis_url = env::var("REDIS_URL")?;
  let manager = RedisConnectionManager::new(&*redis_url)?;
  r2d2::Pool::new(config, manager).map_err(|e| e.into())
}

pub struct RedisConn {
  pub conn: r2d2::PooledConnection<RedisConnectionManager>,
}

impl<'a, 'r> FromRequest<'a, 'r> for RedisConn {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
    let pool = request.guard::<State<Pool>>()?;
    match pool.get() {
      Ok(conn) => Outcome::Success(RedisConn { conn: conn }),
      Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
    }
  }
}

impl Deref for RedisConn {
  type Target = Connection;

  fn deref(&self) -> &Self::Target {
    &self.conn
  }
}
