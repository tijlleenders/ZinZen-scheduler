## ZinZen algorithm

this document contains a high-level overview of the scheduling algorithm

### Given: goals and budgets
we start with defined goals with constraints and modifiers.
We can also have budgets with a minimum and a maximum amount of time to spend on a goal in a given timeframe
(e.g. spend between 4 and 8 hours weekly of studying Rust)

### 1) process goals into steps
goals are processed to form concrete steps. Modifiers are parsed.
e.g. 'run 4 hours every week' generates a step 'run 4 hours between monday and sunday' for every
week between the start and end date

### 2) Generate slots
Go over the timeline and - for each step - calculate all the time slots that are available for that step.

### 3) Calculate flexibility
From the results of the previous step, calculate the flexibility of each step

### 4) Schedule steps with flexibility 1
They can only be scheduled on 1 place: we need to schedule them immediately.
Update flexibility for remaining steps (+ remove slot from those steps).
Decrease budget if the goal corresponding to the step has one,

### 5) If no steps with flexibility 1, schedule the step with the largest flexibility
Schedule it at a timeslot that has no conflict with other unscheduled steps. 
If not possible choose the timeslot with the least conflicts.
Update flexibility for remaining steps (+ remove slot from those steps).
Decrease budget if the goal corresponding to the step has one,


### 6) Repeat step step 4 and 5 until fully scheduled

### 7) See if each minimum budget is reached
if not: generate filler steps for that goal and schedule them, repeating steps 2 to 6 for the newly generated steps.