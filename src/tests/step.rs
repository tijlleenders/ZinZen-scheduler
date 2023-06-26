use crate::models::{
    goal::{Goal, Tag},
    slot::Slot,
    step::{NewStep, Step, StepStatus},
    timeline::Timeline,
};
use chrono::Duration;

#[test]
fn new_step() {
    let step_id = 1;
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

    let new_step = NewStep {
        step_id,
        title: title.clone(),
        duration,
        goal: goal.clone(),
        timeline: timeline.clone(),
        status: status.clone(),
        timeframe,
    };

    let step = Step::new(new_step);

    assert_eq!(step.id, step_id);
    assert_eq!(step.title, title.to_string());
    assert_eq!(step.duration, duration);
    assert_eq!(step.goal_id, goal.id);
    assert_eq!(step.status, status);
    assert_eq!(step.flexibility, 0);
    assert_eq!(step.start, timeframe.map(|t| t.start));
    assert_eq!(step.deadline, timeframe.map(|t| t.end));
    assert_eq!(step.slots, timeline.slots.into_iter().collect::<Vec<_>>());
    assert_eq!(step.tags, goal.tags);
    assert_eq!(step.after_goals, goal.after_goals);
}
