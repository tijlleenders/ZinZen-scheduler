/// Related to #289 https://github.com/tijlleenders/ZinZen-scheduler/issues/289
/// System by default will not split consequent slots which have been split by date.
/// For example for sleep goal from 10pm to 8am, it will be serpated into 2 slots as below:
///     Task: 1
///     Slot: sleep
///     Start: 2022-04-01 22:00
///     End: 2022-04-01 23:59
/// 
///     Task: 1
///     Slot: sleep
///     Start: 2022-04-02 00:00
///     End: 2022-04-02 08:00
/// 
/// New functionality will change the default behvavior, so it will glue slots 
///  which are consequent to each other. So previous exmaple will be one slot:
///     Task: 1
///     Slot: sleep
///     Start: 2022-04-01 22:00
///     End: 2022-04-02 08:00

// ===


/*
TODO 2023-04-12
# Algorithm
- Check check consequent tasks for each goal
- If found consequent tasks:
    - merge them into single task
    - remove 2nd task
    
# Consequent tasks criteria
Tasks considered to be consequent if they meet below criteria:
    - The same task.name for 1st and 2nd consequent task
    - The end-date of 1st task is the same start-date for the 2nd task
    
# Initial Pseudo Code
- for each goal in goals
    - if found consequent tasks
        - merge them into single task
            TODO 2023-04-12 Can we implment a trait to add, subtract tasks from each other
            - Add duration of 2nd task to 1st task
            - Edit 1st task end-date to be 2nd task end-date
        - remove 2nd task
        

*/