use chrono::Duration;
use chrono::prelude::*;
use errors;
use jwt::{encode, decode, Header, Validation};
use rocket::Request;
use rocket::http::Status;
use rocket::outcome;
use rocket::request::{FromRequest, Outcome};
use std::str::FromStr;

#[derive(Debug)]
struct Bearer {
  token: String,
}

impl FromStr for Bearer {
  type Err = errors::Error;

  fn from_str(s: &str) -> errors::Result<Self> {
    let index = s.find("Bearer ")
      .ok_or_else(|| errors::ErrorKind::InvalidAuthorizationHeader(s.to_owned()))?;
    if index > 0 {
      bail!(errors::ErrorKind::InvalidAuthorizationHeader(s.to_owned()))
    }

    let token = &s["Bearer ".len()..];

    Ok(Bearer { token: token.to_owned() })
  }
}

static JWT_SECRET: &'static str = "VERY_SECRET";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub iss: String,
  pub sub: usize,
  pub iat: i64,
  pub exp: i64,
}

impl Claims {
  pub fn issue(sub: usize) -> Claims {
    let utc: DateTime<Utc> = Utc::now();
    let iat = utc.timestamp();
    let exp = (utc + Duration::days(7)).timestamp();
    let iss = "Workout Auth Server".to_owned();

    Claims {
      iss: iss,
      sub: sub,
      iat: iat,
      exp: exp,
    }
  }

  pub fn encode(self) -> errors::Result<String> {
    encode(&Header::default(), &self, JWT_SECRET.as_bytes()).map_err(|err| err.into())
  }

  fn ensure(self, token: String) -> errors::Result<Self> {
    let now = Utc::now().timestamp();
    if self.exp <= now {
      Err(errors::ErrorKind::ExpiredToken(token).into())
    } else {
      Ok(self)
    }
  }
}

impl FromStr for Claims {
  type Err = errors::Error;

  fn from_str(s: &str) -> errors::Result<Self> {
    let token = decode::<Claims>(s, JWT_SECRET.as_bytes(), &Validation::default())?;
    Ok(token.claims)
  }
}

#[derive(Debug)]
pub struct User {
  pub id: usize,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
  type Error = errors::Error;

  fn from_request(request: &'a Request) -> Outcome<Self, Self::Error> {
    let header_map = request.headers();
    let claims_result = header_map.get_one("Authorization")
      .ok_or_else(|| errors::ErrorKind::MissingAuthorizationHeader.into())
      .and_then(|value| value.parse::<Bearer>())
      .and_then(|bearer| {
        bearer.token.parse::<Claims>().map(|claims| (bearer.token.clone(), claims))
      })
      .and_then(|(token, claims)| claims.ensure(token));

    match claims_result {
      Ok(claims) => {
        outcome::Outcome::Success(User {
          id: claims.sub
        })
      },
      Err(e) => {
        outcome::Outcome::Failure((Status::Unauthorized, e))
      }
    }
  }
}
