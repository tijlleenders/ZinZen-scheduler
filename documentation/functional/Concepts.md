## Concepts in the ZinZen&reg; scheduling algorithm
To create a ubiquitous language for talking about the algorithm, all concepts used in the algorithm are defined below.  
To move this doc page to Cargo docs there is [issue #414](https://github.com/tijlleenders/ZinZen-scheduler/issues/414).  

## Overall Concept

The scheduler algorithm is just a transformation function forged into a WASM.

**The scheduler translates the users goals into scheduled tasks.**

### 0) Calendar

The calendar is the overaching datastructure which contains all scheduled tasks from a start date to an end date.

In general, it helps calculating where Activities are occupied by tasks and results in possibly unplaceable tasks.

### 2) Goal

A Goal is the most important concept in ZinZen&reg;.

A Goal is a description of something you want to get done. This can be small, like 'walk 4 hours' - or big like 'Protect the oceans from overfishing'. Goals come from the frontend/UI and are specified by the user.

Goals are organized together with Budgets in a Directed Acyclical Graph (DAG) and have (optional) attributes:  
- Identity - A uuid generated in the frontend.  
- Title - The title. This is necessary only for easier debugging.  
- (Children) - The sub-goals 'in' this Goal.  
- Duration - A duration. Without this, the goal can be transparent in the DAG.  
- (Repeat) - The number of repeats. This translates into number of Activities to generate.  
- (Repeat interval) - Time between the repeats (x hours/days/weeks/months/years).  
- (Dependencies):  
  - Starts:  
    - DateTime after which this should start. Defaults to midnight if no time chosen.
    - Goal with which this should start  
    For example: I can only 'Cook dinner' _after_ I 'Do shopping'.  
  - Ends with:  
    - DateTime. Defaults to midnight if no time chosen.  
    - Number hours spent - For example, consider the goal 'Write first draft of report' completed after investing 3 hours.  
- (Not on) - A collection of Activities that are not allowed to be used.



### 3) Budget

Budgets reserve time on your calendar for a certain purpose.  
This time can be used by any Goals that are children of the Budget in the DAG.  

Budgets share some (optional) attributes with Goals:
- Identity  
- Title  
- (Children)
- (Dependencies)  
- (Not on)

They also have (optional) attributes specific to Budgets:
- Time of day - A pair of [0-23] numbers:
  - After time 
  - Before time  
    If after time is greater than the before time, for example 'Sleep 22-6', the resulting Step Timeline Activitie will span midnight.  
- On days - The days of the week the Budget is allowed to use.
- Min hours per day
- Max hours per day
- Min hours per week 
- Max hours per week  
The min-max per week has to be compatible with the min-max per day in combination with the 'On days'.

### 4) Activity

Goals and Budgets are broken down and represented as activities in the Calendar.

### 5) Task

Tasks are only relevant once _all_ scheduling is done.  
At that point all scheduled Activities are either impossible or scheduled.  

The Activities are then transformed into Tasks: 
- Every Activity becomes a Task
- Any Tasks for that 'touch' AND have the same Goal should be merged.  

