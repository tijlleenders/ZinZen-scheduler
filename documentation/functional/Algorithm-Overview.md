## ZinZen algorithm

This document contains a high-level overview of the scheduling algorithm.  

### 1) The Calendar is created

... and allocates Hours from start to end date.  
Each Hour on the Calendar can be Free or Occupied.

### 2) Activities are created from Goals/Budgets
Each Goal (simple or budget) gets translated to one or more Activities by activity_generator.rs.
Each Activity has an internal calendar_overlay of Vec<Option<Weak>>.  
This get filtered on Activity creation for any Hour that doesn't fit the Goal/Budget constraints.  

After this point, the activity_placer doesn't know anything about dates/times - it just looks at how many blocks are available, and schedules each Activity in turn, counting conflicts via Rc::weak_count, updating the Calendar, which auto-updates the Activities overlay via the Rc:Weak that get invalidated when an Hour is converted from Free to Occupied.

### 3) Simple Goal Activities are scheduled
loop {
  Calculate flex of each Activity.  
  If there is an Activity with flex = 1 ; schedule that
  else, schedule the Activity with the highest flexibility  
    in the place with the least conflict with other Activities
}

### 3) Budget Goal Activities are calculated

Same loop as in 3

### 5) Calendar print 
Sequentially step through all the Hours on the Calendar and group sequential hours occupied by the same Activity as a Task.


## Why this algorithm?
If you'r not convinced this algorithm is good - please suggest a better one. I'm all ears!  
Here's a short explanation / test case to show that giving priority to least flexible Steps is wrong:     
<img src="/documentation/functional/the-wrong-way-of-scheduling.png" alt="The-wrong-way" width="800"/>  

To make it extra clear, here is the correct way of scheduling:
<img src="/documentation/functional/the-correct-way-of-scheduling.png" alt="The-correct-way" width="800"/>  
