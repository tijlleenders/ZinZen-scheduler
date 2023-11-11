## ZinZen algorithm

This document contains a high-level overview of the scheduling algorithm.  
It has two parts:  
- Preparation of Steps and Budgets
- Running the 'placing' algorithm  


## Preparation of Steps and Budgets

### Given: Goals and Budgets
We start with a Directed Acyclical Graph (DAG) of Goals and Budgets, like a tree.  
See [Concepts](./Concepts.md) for more info on Goals and Budgets.

### 1) Add filler Goals if required  
Only the Goals at the 'bottom of the DAG', the 'leaves of the tree', will be considered for generating Filler Goals. 
Sometimes a leaf Goal does not 'consume' all the hours of its parent Goal.  
In those cases a 'filler Goal' is required as a 'brother/sister' of the leaf Goal to add the remaining hours.  
Example:  
Goal 'Project X 20 hours' has a _only_ a subGoal 'Make project planning 3 hours' and nothing else.  
A 'filler Goal' of 'Project X 17 hours' is required next to the planning subGoal.
  
### 2) Extract Budgets
Extract any Budgets.  
All the Steps generated from the subGoals/children of a Budget need to comply to the same Budget - and that's why they need a single Budget coordination point.  
A single Goal can have multiple Budgets. A Budget can impact multiple Goals.   
  
### 3) Generate Steps from the DAG
See [Concepts](./Concepts.md) for more info on Steps.

## Running the 'placing' algorithm

### 1) Calculate flexibility
Calculate the flexibility of each Step, based upon the currently available Timeline with possible Slots.

### 2) Schedule Steps with flexibility 1
They can only be scheduled on 1 place: we need to schedule them immediately.

### 3) If no Steps with flexibility 1, schedule the Step with the largest flexibility
Schedule it at a timeslot that has no conflict with other unscheduled Steps. 
If not possible choose the timeslot with the least conflicts.  
Only schedule if the chosen Slot respects the Budgets registered on the Step.

### 4) Update Timeline of other Steps where needed  
Remove the scheduled Slot(s) from the Timeline of any Steps that still has (part of) the Slot.


### 5) Update Budgets and Budget Steps  

Decrease Budget(s) - if the Step has any.    
These Budgets also generated a set of minimum and optional Steps. These Steps need to be removed accordingly to avoid scheduling 'double'.


### 6) Repeat 1-5 until fully scheduled or impossible


### Why this algorithm?
If you'r not convinced this algorithm is good - please suggest a better one. I'm all ears!  
Here's a short explanation / test case to show that giving priority to least flexible Steps is wrong:     
<img src="/documentation/functional/the-wrong-way-of-scheduling.png" alt="The-wrong-way" width="800"/>  

To make it extra clear, here is the correct way of scheduling:
<img src="/documentation/functional/the-correct-way-of-scheduling.png" alt="The-correct-way" width="800"/>  
