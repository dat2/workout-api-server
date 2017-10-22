#![feature(custom_derive, decl_macro, plugin)]
#![plugin(rocket_codegen)]
#![recursion_limit="128"]

extern crate bcrypt;
extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate jsonwebtoken as jwt;
extern crate rocket;
extern crate rocket_contrib;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod auth;
mod conn;
mod db;
mod errors;
mod schema;
mod models;

use conn::DbConn;
use dotenv::dotenv;
use rocket::request::Form;
use rocket::response::NamedFile;
use rocket_contrib::Json;
use std::convert::From;
use std::io;
use std::path::{Path, PathBuf};

// static files
#[get("/")]
fn index() -> io::Result<NamedFile> {
  NamedFile::open("static/index.html")
}

#[get("/<path..>")]
fn static_dir(path: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("static/").join(path)).ok()
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
  token: String,
}

// register
#[derive(FromForm)]
struct RegisterForm {
  email: String,
  username: String,
  password: String,
}

#[post("/register", data = "<form>")]
fn register(conn: DbConn, form: Form<RegisterForm>) -> errors::Result<Json<Token>> {
  let form: RegisterForm = form.into_inner();

  let existing_users = db::find_users_with_email(&*conn, &form.email)?;
  if !existing_users.is_empty() {
    bail!(errors::ErrorKind::EmailAlreadyRegistered(form.email));
  }

  let user = db::create_user(&*conn, &form.email, &form.username, &form.password)?;
  let token = auth::issue_token(user.id)?;
  Ok(Json(Token { token: token }))
}

// login
#[derive(Debug, FromForm)]
struct LoginForm {
  username: String,
  password: String,
}

#[post("/login", data = "<form>")]
fn login(conn: DbConn, form: Form<LoginForm>) -> errors::Result<Json<Token>> {
  let form: LoginForm = form.into_inner();
  let user = db::find_user(&*conn, &form.username, &form.password)?;
  let token = auth::issue_token(user.id)?;
  Ok(Json(Token { token: token }))
}

#[derive(Debug, Serialize)]
struct Routine {
  id: usize,
  name: String,
  exercises: Vec<Exercise>,
}

impl From<(models::Routine, Vec<models::Exercise>)> for Routine {
  fn from((model, exercise_models): (models::Routine, Vec<models::Exercise>)) -> Self {
    let mut exercises = Vec::new();
    for exercise_model in exercise_models {
      exercises.push(Exercise::from(exercise_model))
    }

    Routine {
      id: model.id as usize,
      name: model.name,
      exercises: exercises
    }
  }
}

#[derive(Debug, Serialize)]
struct Exercise {
  id: usize,
  name: String,
  sets: usize,
  reps: usize,
}

impl From<models::Exercise> for Exercise {
  fn from(model: models::Exercise) -> Exercise {
    Exercise {
      id: model.id as usize,
      name: model.name,
      sets: model.sets as usize,
      reps: model.reps as usize,
    }
  }
}

#[get("/routines", format = "application/json")]
fn list_routines(conn: DbConn) -> errors::Result<Json<Vec<Routine>>> {
  let routines = db::find_routines(&*conn)?;

  let mut result = Vec::new();
  for model in routines.into_iter() {
    result.push(Routine::from(model))
  }
  Ok(Json(result))
}

fn run() -> errors::Result<()> {

  dotenv()?;

  let pool = conn::pool()?;

  rocket::ignite()
    .manage(pool)
    .mount("/", routes![index])
    .mount("/static", routes![static_dir])
    .mount("/api", routes![register, login, list_routines])
    .launch();

  Ok(())
}

quick_main!(run);
