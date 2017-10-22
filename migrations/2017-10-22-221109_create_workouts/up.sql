-- Your SQL goes here
create table workouts(
  id serial primary key,
  user_id integer not null references users (id),
  routine_id integer not null references routines (id),
  created timestamp with time zone not null
);

create table workout_sets(
  id serial primary key,
  workout_id integer not null references workouts (id),
  exercise_id integer not null references exercises (id),
  index integer not null,
  reps integer not null
)
