-- create exercises
insert into exercises (name, sets, reps)
values
  ('Squat', 5, 5),
  ('Bench Press', 5, 5),
  ('Deadlift', 1, 5),
  ('Overhead Press', 5, 5),
  ('Barbell Row', 5, 5);

-- create routines
insert into routines (name)
values
  ('Workout A'),
  ('Workout B');

-- routine a
insert into routine_exercises (routine_id, exercise_id, index)
select routines.id as routine_id, exercises.id as exercise_id, 1 as index
from exercises, routines
where exercises.name = 'Squat' and routines.name = 'Workout A';

insert into routine_exercises (routine_id, exercise_id, index)
select routines.id as routine_id, exercises.id as exercise_id, 2 as index
from exercises, routines
where exercises.name = 'Bench Press' and routines.name = 'Workout A';

insert into routine_exercises (routine_id, exercise_id, index)
select routines.id as routine_id, exercises.id as exercise_id, 3 as index
from exercises, routines
where exercises.name = 'Barbell Row' and routines.name = 'Workout A';

-- routine b
insert into routine_exercises (routine_id, exercise_id, index)
select routines.id as routine_id, exercises.id as exercise_id, 1 as index
from exercises, routines
where exercises.name = 'Squat' and routines.name = 'Workout B';

insert into routine_exercises (routine_id, exercise_id, index)
select routines.id as routine_id, exercises.id as exercise_id, 2 as index
from exercises, routines
where exercises.name = 'Overhead Press' and routines.name = 'Workout B';

insert into routine_exercises (routine_id, exercise_id, index)
select routines.id as routine_id, exercises.id as exercise_id, 3 as index
from exercises, routines
where exercises.name = 'Deadlift' and routines.name = 'Workout B';

