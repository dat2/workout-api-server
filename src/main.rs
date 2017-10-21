#![feature(plugin, custom_derive, decl_macro)]
#![plugin(rocket_codegen)]

extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate jsonwebtoken as jwt;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod auth;
mod errors;

use auth::{Claims, User};
use chrono::prelude::*;
use rocket::request::Form;
use rocket_contrib::Json;
use std::collections::HashMap;

// login
#[derive(FromForm)]
struct LoginRequest {
  user: String,
  pass: String,
}

#[post("/login", data = "<login>")]
fn login(login: Form<LoginRequest>) -> errors::Result<String> {
  let request: LoginRequest = login.into_inner();
  if request.user == "nick" && request.pass == "dujay" {
    Claims::issue(1).encode()
  } else {
    Err(errors::ErrorKind::UserOrPasswordNotFound(request.user).into())
  }
}

// workout
#[derive(Debug, Serialize, Deserialize)]
struct Workout {
  id: usize,
  date_time: DateTime<Utc>,
  exercises: Vec<Exercise>,
  sets: HashMap<usize, Vec<Set>>
}

#[get("/workouts", format = "application/json")]
fn list_workouts(user: User) -> Json<Vec<Workout>> {
  Json(Vec::new())
}

#[get("/workouts/<id>", format = "application/json")]
fn get_workout(id: usize) -> Json<Workout> {
  Json(Workout {
    id: id,
    date_time: Utc::now(),
    exercises: Vec::new(),
    sets: HashMap::new()
  })
}

#[post("/workouts", format = "application/json", data = "<workout>")]
fn create_workout(workout: Json<Workout>) {
  println!("{:?}", workout.id);
}

// workout exercises
#[derive(Debug, Serialize, Deserialize)]
struct Set {
  exercise_id: usize,
  reps: usize,
}

#[post("/my/workouts/<workout_id>/sets", format = "application/json", data = "<set>")]
fn create_set(workout_id: usize, set: Json<Set>) {
  println!("{:?}", workout_id);
  print!("{:?}", set);
}

#[put("/my/workouts/<workout_id>/sets", format = "application/json", data = "<set>")]
fn update_set(workout_id: usize, set: Json<Set>) {
  println!("{:?}", workout_id);
  print!("{:?}", set);
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
fn list_custom_exercises() -> Json<Vec<CustomExercise>> {
  Json(Vec::new())
}

fn main() {
  rocket::ignite()
    .mount("/api", routes![login, list_exercises])
    .mount("/api/my",
           routes![list_workouts,
                   get_workout,
                   create_workout,
                   create_set,
                   update_set,
                   list_custom_exercises])
    .launch();
}
