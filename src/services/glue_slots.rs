/// Related to #289 https://github.com/tijlleenders/ZinZen-scheduler/issues/289
/// System by default will split a task in two consecutive tasks in the output - since output is per day.
/// For example for sleep goal from 10pm to 8am the next day, will be split into 2 tasks in output as below:
///     Task: 1
///     Slot: sleep
///     Start: 2022-04-01 22:00
///     End: 2022-04-02 00:00
/// 
///     Task: 1
///     Slot: sleep
///     Start: 2022-04-02 00:00
///     End: 2022-04-02 08:00
/// 
/// BEFORE reaching the output formatter this should be one Task.
/// To achieve this the Tasks time slots need to be merged after they are split by filters etc...
/// So the two Tasks above will be one Task before going to the output formatter to be split.
// ===

/*
TODO 2023-04-12
# Algorithm
- Check check consecutive time slots within each Task
- When? After task_generator and before task_placer
- If found consecutive time slots in a Task:
    - merge them into single time slot
    - remove 2nd time slot
    
# consecutive tasks criteria
Time slots are considered to be consecutive if they meet below criteria:
    - time_slot.end == time_slot2.start
    
# Initial Pseudo Code
- for each task in tasks
    - if found consecutive time slots
        - merge them into single time slot
            TODO 2023-04-12 Can we implment a trait to add, subtract time slots from each other
            Yes there is some initial code for this by Eric...
        - remove 2nd time slot
        

*/
