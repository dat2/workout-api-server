error_chain! {
  foreign_links {
    Bcrypt(::bcrypt::BcryptError);
    DieselConnection(::diesel::ConnectionError);
    DieselResult(::diesel::result::Error);
    DotEnv(::dotenv::Error);
    ParseIntError(::std::num::ParseIntError);
    R2D2InitializationError(::r2d2::InitializationError);
    RedisError(::redis::RedisError);
    SerdeJsonError(::serde_json::Error);
    Utf8(::std::str::Utf8Error);
    VarError(::std::env::VarError);
  }
  errors {
    UsernameOrPasswordIncorrect {
      description("Username or Password incorrect!")
      display("Username or Password incorrect!")
    }
    EmailAlreadyRegistered {
      description("Email already registered!")
      display("Email already registered!")
    }
    UsernameExists {
      description("Username already registered!")
      display("Username already registered!")
    }
  }
}
