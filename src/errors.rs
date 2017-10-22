error_chain! {
  links {
    Jwt(::jwt::errors::Error, ::jwt::errors::ErrorKind);
  }
  foreign_links {
    Bcrypt(::bcrypt::BcryptError);
    DieselConnection(::diesel::ConnectionError);
    DieselResult(::diesel::result::Error);
    DotEnv(::dotenv::Error);
    R2D2InitializationError(::r2d2::InitializationError);
    Utf8(::std::str::Utf8Error);
    VarError(::std::env::VarError);
  }
  errors {
    MissingAuthorizationHeader
    InvalidAuthorizationHeader(value: String) {
      description("invalid authorization header")
      display("invalid authorization header: '{}'", value)
    }
    ExpiredToken(token: String) {
      description("jwt token is expired")
      display("jwt token is expired '{}'", token)
    }
    UserOrPasswordNotCorrect(user: String) {
      description("user or password not correct")
      display("user or password not correct: '{}'", user)
    }
    EmailAlreadyRegistered(email: String) {
      description("email already registered")
      display("email already registered: '{}'", email)
    }
  }
}
