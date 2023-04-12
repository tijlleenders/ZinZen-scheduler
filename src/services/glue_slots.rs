/// Related to https://github.com/tijlleenders/ZinZen-scheduler/issues/289
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


