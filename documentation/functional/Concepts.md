## Concepts in the ZinZen&reg; scheduling algorithm
To create a ubiquitous language for talking about the algorithm, all concepts used in the algorithm are defined below.  
To move this doc page to Cargo docs there is [issue #414](https://github.com/tijlleenders/ZinZen-scheduler/issues/414).  

### 1) Goals
A Goal is the most important concept in ZinZen&reg;, next to Budgets.  
A Goal is a description of something you want to get done. This can be small, like 'walk 4 hours' - or big like 'Protect the oceans from overfishing'. Goals come from the frontend/UI and are specified by the user.

Goals are organized together with Budgets in a Directed Acyclical Graph (DAG) and have (optional) attributes:  
- Identity - A uuid generated in the frontend.  
- Title - The title. This is necessary only for easier debugging.  
- (Children) - The sub-goals 'in' this Goal.  
- Duration - A duration. Without this, the goal can be transparent in the DAG.  
- (Repeat) - The number of repeats. This translates into number of Steps to generate.  
- (Repeat interval) - Time between the repeats (x hours/days/weeks/months/years).  
- (Dependencies):  
  - Starts:  
    - DateTime after which this should start. Defaults to midnight if no time chosen.
    - Goal with which this should start  
    For example: I can only 'Cook dinner' _after_ I 'Do shopping'.  
  - Ends with:  
    - DateTime. Defaults to midnight if no time chosen.  
    - Number hours spent - For example, consider the goal 'Write first draft of report' completed after investing 3 hours.  
- (Not on) - A collection of Slots that are not allowed to be used.

### 2) Budgets  
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
    If after time is greater than the before time, for example 'Sleep 22-6', the resulting Step Timeline Slot will span midnight.  
- On days - The days of the week the Budget is allowed to use.
- Min hours per day
- Max hours per day
- Min hours per week 
- Max hours per week  
The min-max per week has to be compatible with the min-max per day in combination with the 'On days'.


### 2) Steps
Steps are the building blocks for the 'placing' algorithem of the scheduler.  
Important!: Some older terminology and documentation describes this concept as 'Tasks' - but 'Task' is now reserved only for the final output sent to the frontend.

Steps can be generated in two ways:  
- From a Goal:  
  - Make a new Step with corresponding Timeline for every Repeat.
- From a Budget:  
  - Make a new Step for every day interval using the minimum hours per day attribute. 
  - Make a new optional Step for every day interval using the difference between min-max per day.

Steps are organized in a list and have the following (optional) attributes:  
- Duration  
- Timeline - This is a collection of Slots that comply to the constraints set for this Step.  
- Flexibility - This is how many different ways the Step can theoretically be 'placed' in the Timeline. It can be calculated using Duration and Timeline. This needs to be recalculated after every change to the Timeline.
- Type - used by 'placer' together with Flexibility to determine priority:
  - Goal
  - Budget
  - Optional budget
- (Budgets) - A list of Budgets the Step needs to comply with

Example on calculating Step Flexibility:  
A Step with Duration 4 and a Timeline with one Slot of [8-14] can placed in 3 ways:  
- 8-12
- OR 9-13
- OR 10-14  
and thus has a flexibility of 3.

### 3) Slots
Slots are periods of time: [StartDateTime; EndDateTime[.  
Currently the granularity of Slots is in hours. 
A Slot can be 1h long, or max 7*24 hours (one week) long.  
Important!: Slots are not unique:
- Multiple Steps can have similar or overlapping Slots in their Timeline.

### 4) Tasks  
Tasks are only relevant once _all_ scheduling is done.  
At that point all scheduled Steps are either impossible or scheduled.  

The Steps are then transformed into Tasks: 
- Every Step becomes a Task
- Any Tasks for that 'touch' AND have the same Goal should be merged.  