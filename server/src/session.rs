use errors;
use models::User;
use redis::{Commands, Connection};
use redis_conn::RedisConn;
use rocket::Request;
use rocket::http::{Cookie, Cookies, Status};
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome};
use serde_json;

pub struct SessionToken {
  id: i32,
}

impl<'a, 'r> FromRequest<'a, 'r> for SessionToken {
  type Error = ();

  fn from_request(request: &'a Request) -> Outcome<Self, ()> {
    request.cookies()
      .get_private("session")
      .ok_or(())
      .and_then(|c| c.value().parse::<i32>().map_err(|_| ()))
      .map(|id| SessionToken { id: id })
      .into_outcome(Status::BadRequest)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
  pub id: i32,
  pub user_id: i32,
}

impl<'a, 'r> FromRequest<'a, 'r> for Session {
  type Error = ();

  fn from_request(request: &'a Request) -> Outcome<Self, ()> {
    let conn = RedisConn::from_request(request)?;
    let token = SessionToken::from_request(request)?;

    get(&conn, token)
      .map_err(|_| ())
      .into_outcome(Status::BadRequest)
  }
}


pub fn get(conn: &Connection, token: SessionToken) -> errors::Result<Session> {
  let serialized: String = conn.get(format!("session_{}", token.id))?;
  let session: Session = serde_json::from_str(&serialized)?;
  Ok(session)
}

pub fn persist(conn: &Connection, user_id: i32) -> errors::Result<()> {
  let id: i32 = conn.incr("session_id", 1)?;

  let session = Session {
    id: id,
    user_id: user_id,
  };
  let serialized: String = serde_json::to_string(&session)?;
  let _: () = conn.set(format!("session_{}", id), serialized)?;
  Ok(())
}

pub fn add_cookie(cookies: &mut Cookies, user: &User) {
  cookies.add_private(Cookie::build("session", user.id.to_string())
    .path("/api")
    .finish());
}

pub fn remove_cookie(cookies: &mut Cookies) {
  cookies.remove_private(Cookie::named("session"));
}
