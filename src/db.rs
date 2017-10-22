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
  use schema::routines;

  let routines: Vec<Routine> = routines::table.load(conn)?;

  let mut result = Vec::new();
  for routine in routines.into_iter() {
    let exercises = find_exercises_for_routine(conn, &routine)?;
    result.push((routine, exercises))
  }
  Ok(result)
}

fn find_exercises_for_routine(conn: &PgConnection,
                              routine: &Routine)
                              -> errors::Result<Vec<Exercise>> {
  use schema::exercises::dsl::*;
  use schema::routine_exercises::dsl::*;

  let rows: Vec<(RoutineExercise, Exercise)> =
    routine_exercises.filter(routine_id.eq(routine.id))
      .inner_join(exercises)
      .order(index)
      .load(conn)?;

  let mut result = Vec::new();
  for (_, exercise) in rows.into_iter() {
    result.push(exercise);
  }
  Ok(result)
}
