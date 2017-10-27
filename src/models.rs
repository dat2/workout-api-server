use chrono::{DateTime, Utc};
use schema::{users, exercises, routines, routine_exercises, workouts};

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
  pub email: &'a str,
  pub username: &'a str,
  pub password: &'a str,
}

#[derive(Debug, Queryable)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub password: String,
  pub email: String,
}

#[derive(Debug, Identifiable, Queryable, Associations)]
pub struct Routine {
  pub id: i32,
  pub name: String,
}

#[derive(Debug, Identifiable, Queryable, Associations)]
#[primary_key(routine_id, exercise_id)]
#[belongs_to(Routine)]
#[belongs_to(Exercise)]
pub struct RoutineExercise {
  pub routine_id: i32,
  pub exercise_id: i32,
  pub index: i32,
}

#[derive(Debug, Identifiable, Queryable, Associations)]
pub struct Exercise {
  pub id: i32,
  pub name: String,
  pub sets: i32,
  pub reps: i32,
}

#[derive(Debug, Insertable)]
#[table_name="workouts"]
pub struct NewWorkout {
  pub user_id: i32,
  pub routine_id: i32,
  pub created: DateTime<Utc>,
}

#[derive(Debug, Identifiable, Queryable)]
pub struct Workout {
  pub id: i32,
  pub user_id: i32,
  pub routine_id: i32,
  pub created: DateTime<Utc>,
}
