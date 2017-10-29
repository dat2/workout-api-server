-- Your SQL goes here
create table users (
  id serial primary key,
  username text unique not null,
  password text not null,
  email text unique not null
)
