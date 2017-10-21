error_chain! {
  links {
    Jwt(::jwt::errors::Error, ::jwt::errors::ErrorKind);
  }
  foreign_links {
    Utf8(::std::str::Utf8Error);
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
    UserOrPasswordNotFound(user: String) {
      description("user or password not found")
      display("user or password not found: '{}'", user)
    }
  }
}
