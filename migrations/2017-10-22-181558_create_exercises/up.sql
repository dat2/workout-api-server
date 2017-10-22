-- Your SQL goes here
create table exercises(
  id serial primary key,
  name text not null,
  sets integer not null check (sets > 0),
  reps integer not null check (reps > 0)
);

create table routines(
  id serial primary key,
  name text not null
);

create table routine_exercises(
  routine_id integer references routines (id),
  exercise_id integer references exercises (id),
  index integer not null,
  primary key (routine_id, exercise_id, index)
);
