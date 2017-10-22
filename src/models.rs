use schema::users;

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
