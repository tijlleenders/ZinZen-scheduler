## ZinZen algorithm

This document contains a high-level overview of the scheduling algorithm.  
It has two parts:  
- Preparation of Steps and Budgets
- Running the 'placing' algorithm  


## Preparation of Steps and Budgets

### Given: Goals and Budgets
We start with defined Goals with constraints and modifiers.
We can also have Budgets with a minimum and a maximum amount of time to spend on a Goal per day and per week (e.g. spend between 4 to 8 hours weekly of studying Rust, but 0-2 hours per day to keep it effective).  

### 1) Add filler Goals if required  
The Goals and Budgets are presented in a Directed Acyclic Graph (DAG), like a tree. Only the Goals at the 'bottom of the DAG' will be considered for generating Filler Goals. We call these the 'leaves' of the tree.  
Sometimes a leaf Goal does not 'consume' all the hours of its parent Goal.  
In those cases a 'filler Goal' is required as a 'brother/sister' of the leaf Goal to add the remaining hours.  
Example:  
Goal 'Project X 20 hours' has a _only_ a subGoal 'Make project planning 3 hours' and nothing else.  
A 'filler Goal' of 'Project X 17 hours' is required next to the planning subGoal.
  
### 2) Extract Budgets
Extract any Budgets - and register which Goals need to respect which Budgets for which time periods. All the Steps generated from the subGoals/children of a Budget need to comply to the Budget - and that's why they need a single Budget coordination point.  
A single Goal can have multiple Budgets. A Budget can impact multiple Goals.   
  
### 3) Process Goals and Budgets into Steps
Goals are processed to form concrete Steps. Modifiers are parsed.
e.g. 'run 4 hours every week' generates a Step 'run 4 hours between monday and sunday' for every
week between the start and end date.

Goals with durations <= 8 hours are kept as a single block. Goals with a longer duration are split into 1-hour sized Steps.  

Budgets can also generate Steps - irrespective of their place in the hierarchy. They are only placed after all 'regular' Steps are placed.  
NB!: Whenever a 'regular' Step belonging to the Budget is placed, any corresponding Budget Steps get removed for the corresponding time periods and durations.  

### 4) Generate slots
Go over the timeline and - for each Step - calculate all the time slots that are available for that Step.  
These slots can be much larger than the required time; it represents the time slots that are allowed.

## Running the 'placing' algorithm

### 1) Calculate flexibility
Calculate the flexibility of each Step, based upon the currently available Timeline with possible Slots.

### 2) Schedule Steps with flexibility 1
They can only be scheduled on 1 place: we need to schedule them immediately.

### 3) If no Steps with flexibility 1, schedule the Step with the largest flexibility
Schedule it at a timeslot that has no conflict with other unscheduled Steps. 
If not possible choose the timeslot with the least conflicts.

### 4) Update Timeline of other Steps where needed  
Remove the scheduled Slot(s) from the Timeline of any Steps that still had the Slot(s) (or part of it) as 'available'.  


### 5) Update Budgets and Budget Steps  

Decrease Budget if the Goal corresponding to the Step has one, and remove any minimum Budget Steps for corresponding period and duration.


### 6) Repeat 1-5 until fully scheduled or impossible


### 7) See if each maximum Budget is reached
If not: generate filler Steps for that Goal Budget time period and schedule them, repeating 1-5 for the newly generated Steps. Any of these Steps that are impossible to place can be discarded - as these are optional to 'top up' - the minimum Budgets have already been satisfied.



### Why this algorithm?
If you'r not convinced this algorithm is good - please suggest a better one. I'm all ears!  
Here's a short explanation / test case to show that working from most flexible to least flexible works out better in practice:  
<img src="/documentation/functional/why-most-flex-to-least-is-better.png" alt="Why-this-is-better" width="800"/>