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

use auth::{Claims, User};
use chrono::prelude::*;
use conn::DbConn;
use dotenv::dotenv;
use rocket::request::Form;
use rocket_contrib::Json;
use std::collections::HashMap;

// register
#[derive(FromForm)]
struct RegisterForm {
  email: String,
  username: String,
  password: String,
}

#[post("/register", data = "<form>")]
fn register(conn: DbConn, form: Form<RegisterForm>) -> errors::Result<()> {

  let form: RegisterForm = form.into_inner();

  let existing_users = db::find_users_with_email(&*conn, &form.email)?;
  if existing_users.len() > 0 {
    bail!(errors::ErrorKind::EmailAlreadyRegistered(form.email));
  }

  db::create_user(&*conn, &form.email, &form.username, &form.password)?;

  Ok(())
}

// login
#[derive(FromForm)]
struct LoginForm {
  username: String,
  password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenIssued {
  token: String,
}

#[post("/login", data = "<form>")]
fn login(conn: DbConn, form: Form<LoginForm>) -> errors::Result<Json<TokenIssued>> {
  let form: LoginForm = form.into_inner();
  let user = db::find_user(&*conn, &form.username, &form.password)?;
  let token = Claims::issue(user.id).encode()?;
  Ok(Json(TokenIssued { token: token }))
}

// workout
#[derive(Debug, Serialize, Deserialize)]
struct Workout {
  id: usize,
  date_time: DateTime<Utc>,
  exercises: Vec<Exercise>,
  sets: HashMap<usize, Vec<Set>>,
}

#[get("/workouts", format = "application/json")]
fn list_workouts(user: User) -> Json<Vec<Workout>> {
  println!("Getting workouts for user {:?}", user);
  Json(Vec::new())
}

#[get("/workouts/<id>", format = "application/json")]
fn get_workout(user: User, id: usize) -> Json<Workout> {
  println!("Getting workout {} for user {:?}", id, user);
  Json(Workout {
    id: id,
    date_time: Utc::now(),
    exercises: Vec::new(),
    sets: HashMap::new(),
  })
}

#[post("/workouts", format = "application/json", data = "<workout>")]
fn create_workout(user: User, workout: Json<Workout>) {
  println!("Creating workout {:?} for user {:?}", *workout, user);
}

// workout exercises
#[derive(Debug, Serialize, Deserialize)]
struct Set {
  exercise_id: usize,
  reps: usize,
}

#[post("/my/workouts/<workout_id>/sets", format = "application/json", data = "<set>")]
fn create_set(user: User, workout_id: usize, set: Json<Set>) {
  println!("Creating set {:?} for workout {:?} for user {:?}",
           set,
           workout_id,
           user);
}

#[put("/workouts/<workout_id>/sets", format = "application/json", data = "<set>")]
fn update_set(user: User, workout_id: usize, set: Json<Set>) {
  println!("Updating set {:?} for workout {:?} for user {:?}",
           set,
           workout_id,
           user);
}

// exercises
#[derive(Debug, Serialize, Deserialize)]
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

// custom exercises
#[derive(Debug, Serialize, Deserialize)]
struct CustomExercise {
  owner_id: usize,
  id: usize,
  name: String,
  sets: usize,
  reps: usize,
}

#[get("/exercises", format = "application/json")]
fn list_custom_exercises(user: User) -> Json<Vec<CustomExercise>> {
  println!("Listing custom exercises for user {:?}", user);
  Json(Vec::new())
}

fn run() -> errors::Result<()> {

  dotenv()?;

  let pool = conn::pool()?;

  rocket::ignite()
    .manage(pool)
    .mount("/api", routes![register, login, list_exercises])
    .mount("/api/my",
           routes![list_workouts,
                   get_workout,
                   create_workout,
                   create_set,
                   update_set,
                   list_custom_exercises])
    .launch();

  Ok(())
}

quick_main!(run);
