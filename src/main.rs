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

use auth::UserId;
use chrono::prelude::*;
use conn::DbConn;
use dotenv::dotenv;
use rocket::request::Form;
use rocket::response::{NamedFile, status};
use rocket_contrib::Json;
use std::collections::HashMap;
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
struct Workout {
  id: usize,
  exercises: Vec<Exercise>,
}

#[get("/workouts", format = "application/json")]
fn list_workouts(user_id: UserId) -> Json<Vec<Workout>> {
  println!("Getting workouts for user {:?}", user_id);
  Json(Vec::new())
}

#[get("/workouts/<workout_id>", format = "application/json")]
fn get_workout(user_id: UserId, workout_id: usize) -> Json<Workout> {
  println!("Getting workout {} for user {:?}", workout_id, user_id);
  Json(Workout {
    id: workout_id,
    exercises: Vec::new(),
  })
}

#[derive(Debug, Serialize, Deserialize)]
struct NewWorkout {
  exercises: HashMap<i32, Vec<Vec<usize>>>,
}

#[post("/workouts", format = "application/json", data="<new_workout>")]
fn create_workout(user_id: UserId,
                  new_workout: Json<NewWorkout>)
                  -> errors::Result<status::Created<Json<Workout>>> {
  println!("Creating new_workout {:?} for user {:?}",
           *new_workout,
           user_id);
  let json = Json(Workout {
    id: 1,
    exercises: Vec::new(),
  });

  Ok(status::Created(format!("/api/workouts/{}", 1), Some(json)))
}

// exercises
#[derive(Debug, Serialize)]
struct Exercise {
  id: usize,
  name: String,
  sets: usize,
  reps: usize,
}

#[get("/exercises", format = "application/json")]
fn list_exercises() -> Json<Vec<Exercise>> {
  Json(Vec::new())
}

fn run() -> errors::Result<()> {

  dotenv()?;

  let pool = conn::pool()?;

  rocket::ignite()
    .manage(pool)
    .mount("/", routes![index])
    .mount("/static", routes![static_dir])
    .mount("/api", routes![register, login, list_exercises])
    .mount("/api/my",
           routes![list_workouts, get_workout, create_workout])
    .launch();

  Ok(())
}

quick_main!(run);
