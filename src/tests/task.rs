use crate::models::{
    goal::{Goal, Tag},
    slot::Slot,
    task::{NewStep, Step, StepStatus},
    timeline::Timeline,
};
use chrono::Duration;

#[test]
fn new_task() {
    let task_id = 1;
    let title = "Do laundry".to_string();
    let duration = 2;
    let goal = Goal {
        id: "1".to_string(),
        title: title.to_string(),
        tags: vec![Tag::Budget],
        after_goals: None,
        ..Default::default()
    };
    let timeline = Timeline::new();
    let status = StepStatus::ReadyToSchedule;
    let timeframe = Some(Slot::mock(Duration::days(2), 2023, 05, 01, 0, 0));

    let new_task = NewStep {
        task_id,
        title: title.clone(),
        duration,
        goal: goal.clone(),
        timeline: timeline.clone(),
        status: status.clone(),
        timeframe,
    };

    let task = Step::new(new_task);

    // let task = Task::new(
    //     task_id, title, duration, &goal, &timeline, &status, timeframe,
    // );

    assert_eq!(task.id, task_id);
    assert_eq!(task.title, title.to_string());
    assert_eq!(task.duration, duration);
    assert_eq!(task.goal_id, goal.id);
    assert_eq!(task.status, status);
    assert_eq!(task.flexibility, 0);
    assert_eq!(task.start, timeframe.map(|t| t.start));
    assert_eq!(task.deadline, timeframe.map(|t| t.end));
    assert_eq!(task.slots, timeline.slots.into_iter().collect::<Vec<_>>());
    assert_eq!(task.tags, goal.tags);
    assert_eq!(task.after_goals, goal.after_goals);
}
