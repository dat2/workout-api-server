use chrono::Duration;
use chrono::prelude::*;
use errors;
use jwt::{encode, decode, Header, Validation};
use rocket::Request;
use rocket::http::Status;
use rocket::outcome;
use rocket::request::{FromRequest, Outcome};
use std::env;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub iss: String,
  pub sub: i32,
  pub iat: i64,
  pub exp: i64,
}

impl Claims {
  fn encode(self) -> errors::Result<String> {
    let secret = env::var("JWT_SECRET")?;
    encode(&Header::default(), &self, secret.as_bytes()).map_err(|err| err.into())
  }

  fn decode(s: &str) -> errors::Result<Claims> {
    let secret = env::var("JWT_SECRET")?;
    let token = decode::<Claims>(s, secret.as_bytes(), &Validation::default())?;
    Ok(token.claims)
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

pub fn issue_token(sub: i32) -> errors::Result<String> {
  let utc: DateTime<Utc> = Utc::now();
  let iat = utc.timestamp();
  let exp = (utc + Duration::days(7)).timestamp();
  let iss = "Workout Auth Server".to_owned();

  let claims = Claims {
    iss: iss,
    sub: sub,
    iat: iat,
    exp: exp,
  };
  claims.encode()
}

#[derive(Debug)]
pub struct UserId {
  pub id: i32,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserId {
  type Error = errors::Error;

  fn from_request(request: &'a Request) -> Outcome<Self, Self::Error> {
    let header_map = request.headers();
    let claims_result = header_map.get_one("Authorization")
      .ok_or_else(|| errors::ErrorKind::MissingAuthorizationHeader.into())
      .and_then(|value| value.parse::<Bearer>())
      .and_then(|bearer| Claims::decode(&bearer.token).map(|claims| (bearer.token.clone(), claims)))
      .and_then(|(token, claims)| claims.ensure(token));

    match claims_result {
      Ok(claims) => outcome::Outcome::Success(UserId { id: claims.sub }),
      Err(e) => outcome::Outcome::Failure((Status::Unauthorized, e)),
    }
  }
}
