error_chain! {
  foreign_links {
    Bcrypt(::bcrypt::BcryptError);
    DieselConnection(::diesel::ConnectionError);
    DieselResult(::diesel::result::Error);
    DotEnv(::dotenv::Error);
    ParseIntError(::std::num::ParseIntError);
    R2D2InitializationError(::r2d2::InitializationError);
    Utf8(::std::str::Utf8Error);
    VarError(::std::env::VarError);
  }
  errors {
    MissingCookie
    UserOrPasswordIncorrect(user: String) {
      description("user or password incorrect")
      display("user or password incorrect: '{}'", user)
    }
    EmailAlreadyRegistered(email: String) {
      description("email already registered")
      display("email already registered: '{}'", email)
    }
  }
}
