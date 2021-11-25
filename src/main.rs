use zinzen_scheduler::*;

fn main() {
    let mut calendar = Calendar::new(168, String::from("h"));

    let goal = Goal {
        id: 1,
        title: String::from("daily goal"),
        estimated_duration: 1,
        effort_invested: 0,
        start: 0,
        finish: 8760, //one year
        start_time: 12,
        finish_time: 13,
        goal_type: GoalType::DAILY,
    };

    let goal2 = Goal {
        id: 2,
        title: String::from("imp daily goal"),
        estimated_duration: 1,
        effort_invested: 0,
        start: 0,
        finish: 168,
        start_time: 12,
        finish_time: 13,
        goal_type: GoalType::DAILY,
    };
    calendar.add(goal);
    calendar.add(goal2);

    // print!("Calendar:{:#?}\n", calendar);

    // print!("\nexpect Calendar with two goals not overlapping\n");
    calendar.schedule();

    // calendar.print_slots_for_range(12, 14);
    print!("Calendar:{:#?}\n", calendar);
}
