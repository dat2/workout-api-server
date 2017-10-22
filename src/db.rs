use bcrypt::{DEFAULT_COST, hash, verify};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use errors;
use models::{NewUser, User, Routine, RoutineExercise, Exercise};

pub fn find_user(conn: &PgConnection,
                 query_username: &str,
                 query_password: &str)
                 -> errors::Result<User> {
  use schema::users::dsl::*;

  let user = users.filter(username.eq(query_username))
    .get_result::<User>(conn)?;

  let password_matches = verify(query_password, &user.password)?;
  if !password_matches {
    bail!(errors::ErrorKind::UserOrPasswordNotCorrect(query_username.to_owned()));
  }

  Ok(user)
}

pub fn find_users_with_email(conn: &PgConnection, query_email: &str) -> errors::Result<Vec<User>> {
  use schema::users::dsl::*;

  users.filter(email.eq(query_email))
    .load::<User>(conn)
    .map_err(|e| e.into())
}

pub fn create_user<'a>(conn: &PgConnection,
                       new_email: &'a str,
                       new_username: &'a str,
                       new_password: &'a str)
                       -> errors::Result<User> {
  use schema::users;

  let hashed_password = hash(new_password, DEFAULT_COST)?;
  let new_user = NewUser {
    email: new_email,
    username: new_username,
    password: &hashed_password,
  };

  diesel::insert(&new_user)
    .into(users::table)
    .get_result(conn)
    .map_err(|e| e.into())
}

pub fn find_routines(conn: &PgConnection) -> errors::Result<Vec<(Routine, Vec<Exercise>)>> {
  use schema::exercises;
  use schema::routines;
  use schema::routine_exercises;

  let routines: Vec<Routine> = routines::table.load(conn)?;
  let routine_exercises: Vec<(RoutineExercise, Exercise)> = RoutineExercise::belonging_to(&routines)
    .order(routine_exercises::index)
    .inner_join(exercises::table)
    .load(conn)?;
  let grouped_routine_exercises = routine_exercises.grouped_by(&routines)
    .into_iter()
    .map(|vec| vec.into_iter().map(|tuple| tuple.1).collect());
  Ok(routines.into_iter().zip(grouped_routine_exercises).collect())
}
