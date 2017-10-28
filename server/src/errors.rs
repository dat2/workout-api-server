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
