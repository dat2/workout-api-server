use errors;
use models::User;
use rocket::Request;
use rocket::http::{Cookie, Cookies, Status};
use rocket::outcome;
use rocket::request::{FromRequest, Outcome};

#[derive(Debug)]
pub struct UserId {
  pub id: i32,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserId {
  type Error = errors::Error;

  fn from_request(request: &'a Request) -> Outcome<Self, Self::Error> {
    let result = request.cookies()
      .get_private("user_id")
      .ok_or_else(|| errors::ErrorKind::MissingCookie.into())
      .and_then(|cookie| cookie.value().parse::<i32>().map_err(|e| e.into()));

    match result {
      Ok(user_id) => outcome::Outcome::Success(UserId { id: user_id }),
      Err(e) => outcome::Outcome::Failure((Status::Unauthorized, e)),
    }
  }
}

pub fn add_user_cookie(cookies: &mut Cookies, user: &User) {
  cookies.add_private(Cookie::new("user_id", user.id.to_string()))
}
