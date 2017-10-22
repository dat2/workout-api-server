use schema::{users, exercises, routines};

#[derive(Insertable)]
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

#[derive(Debug, Identifiable, Queryable)]
pub struct Exercise {
  pub id: i32,
  pub name: String,
  pub sets: i32,
  pub reps: i32,
}

#[derive(Debug, Identifiable, Queryable)]
pub struct Routine {
  pub id: i32,
  pub name: String,
}

#[derive(Debug, Queryable)]
pub struct RoutineExercise {
  pub routine_id: i32,
  pub exercise_id: i32,
  pub index: i32,
}
