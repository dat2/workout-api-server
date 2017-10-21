#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use chrono::prelude::*;
use rocket_contrib::Json;

// workout
#[derive(Debug, Serialize, Deserialize)]
struct Workout {
  id: usize,
  date_time: DateTime<Utc>
}

#[get("/workouts")]
fn list_workouts() -> Json<Vec<Workout>> {
  Json(Vec::new())
}

#[get("/workouts/<id>", format = "application/json")]
fn get_workout(id: usize) -> Json<Workout> {
  Json(Workout {
    id: id,
    date_time: Utc::now()
  })
}

#[post("/workouts", format = "application/json", data = "<workout>")]
fn create_workout(workout: Json<Workout>) -> () {
  println!("{:?}", workout.id);
  ()
}

fn main() {
  rocket::ignite()
    .mount("/", routes![
      list_workouts,
      get_workout,
      create_workout
    ])
    .launch();
}
