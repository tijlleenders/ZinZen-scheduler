## ZinZen algorithm

this document contains a high-level overview of the scheduling algorithm

### Given: goals and budgets
we start with defined goals with constraints and modifiers.
We can also have budgets with a minimum and a maximum amount of time to spend on a goal in a given timeframe
(e.g. spend between 4 and 8 hours weekly of studying Rust)
  
## 0) Add filler Goals if required  
The Goals are presented in a Directed Acyclic Graph (DAG), like a tree. Only the Goals at the 'bottom of the DAG' will be considered for generating Steps. We call these the 'leaves' of the tree.
Sometimes a leaf Goal does not 'consume' all the hours of its parent Goal.  
In those cases a 'filler Goal' is required as a 'brother/sister' of the leaf Goal to add the remaining hours.  
Example:  
Goal 'Project X 20 hours' has a _only_ a subGoal 'Make project planning 3 hours' and nothing else.  
A 'filler Goal' of 'Project X 17 hours' is required next to the planning subGoal.
  
### 1)
Extract any budgets - and register which Goals need to respect which budgets.  
A budget can be specified 'on' the Goal itself, or on any of the parents/ancestors in the DAG.  
A single Goal can have multiple budgets. A budget can impact multiple Goals.
  
### 2) process goals into steps
goals are processed to form concrete steps. Modifiers are parsed.
e.g. 'run 4 hours every week' generates a step 'run 4 hours between monday and sunday' for every
week between the start and end date

### 3) Generate slots
Go over the timeline and - for each step - calculate all the time slots that are available for that step.

### 4) Calculate flexibility
From the results of the previous step, calculate the flexibility of each step

### 5) Schedule steps with flexibility 1
They can only be scheduled on 1 place: we need to schedule them immediately.
Update flexibility for remaining steps (+ remove slot from those steps).
Decrease budget if the goal corresponding to the step has one,

### 6) If no steps with flexibility 1, schedule the step with the largest flexibility
Schedule it at a timeslot that has no conflict with other unscheduled steps. 
If not possible choose the timeslot with the least conflicts.
Update flexibility for remaining steps (+ remove slot from those steps).
Decrease budget if the goal corresponding to the step has one,


### 7) Repeat step step 5 and 6 until fully scheduled

### 8) See if each minimum budget is reached
if not: generate filler steps for that goal and schedule them, repeating steps 2 to 6 for the newly generated steps.