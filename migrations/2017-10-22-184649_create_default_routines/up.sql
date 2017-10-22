-- create exercises
insert into exercises (id, name, sets, reps)
values
  (default, 'Squat', 5, 5),
  (default, 'Bench Press', 5, 5),
  (default, 'Deadlift', 1, 5),
  (default, 'Overhead Press', 5, 5),
  (default, 'Barbell Row', 5, 5);

-- create routines
insert into routines (id, name)
values
  (default, 'Workout A'),
  (default, 'Workout B');

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

