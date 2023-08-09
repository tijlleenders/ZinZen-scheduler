## Concepts in the ZinZen&reg; scheduling algorithm
To create a ubiquitous language for talking about the algorithm, all concepts used in the algorithm are defined below.

### 1) Goals
A goal is the most important concept in ZinZen&reg;. A goal is a description of something 
you want to get done, like 'walk 4 hours'. Goals come from the frontend/UI and are specified by the user.

Goals can be modified with restrictions and repeats, like 'walk 4 hours every week between 6p.m. and 10p.m from Sep. 1st until Oct. 1st'

### 2) Steps
Steps are concrete steps to reach a goal with all it's restrictions and modifiers.

They contain a start time, a deadline and a duration.

The example above ('walk 4 hours every week between 6p.m. and 10p.m from Sep. 1st until Oct. 1st')
can be split into multiple Steps of a week each, beginning with the start date-time and incrementing in 'steps' of 7 day intervals. The last Step can be shortened if the deadline is reached.
day of the week and a duration of 4 hours.

n.b. some older terminology and documentation describes this concept as 'Tasks'.

### 3) Tasks  
Tasks are only relevant once _all_ scheduling is done. At that point all scheduled Steps are transformed into Tasks. A Task is something you'll see on the final calendar.  
Since not all scheduled Slots of a Step will be next to each other, a Task is necessary for each combination of Slots that are next to each other.  
Sometimes a Step only has one Slot, so then only one Task. If a Step has two scheduled Slots, but they are next to each other, this still is only one Task.

### 4) Slots
Slots are units of time in which steps can be planned. The current implementation of the algorithm
divides the universe into slots of 1 hour. A Slot can be occupied (used by a Step) or free. A single Slot can only be occupied by one Step.  
Multiple Steps could 'want' the same Slot.

### 5) Flexibility
Each step - at any point of the algorithm execution - has a 'flexibility score'. This number represents
in how many different ways the desired step can be planned in the available slots, given constraints.

Example:  
A 4-hour step that has to be scheduled between 8h and 14h a given day can be placed in 3 ways
(8:00-12:00 OR 09:00-13:00 OR 10:00-14:00) and thus has a flexibility of 3.

### 6) Budget
A budget is a minimum number of hours _required_ to be spent in a given timeframe on a goal, and a maximum number of hours _if possible_ for that same timeframe.

Example:  
On goal 'learn Spanish' I want to spend a minimum of 4 hours and a maximum of 8 hours every week.
This is represented as a budget with:
* minimum 4h
* maximum 6h
* timeframe weekly