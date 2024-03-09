#[cfg(test)]
mod TEST_MODULE_NAME {

    // stable tests
    //TEST_FUNCTIONS_STABLE

    // experimental tests
    //TEST_FUNCTIONS_EXPERIMENTAL

    use crate::calendar::Calendar;
    use crate::Input;
    use scheduler::models::activity::Activity;
    use scheduler::services::activity_generator::adjust_parent_activities;
    use scheduler::services::{activity_generator, activity_placer};

    use scheduler::technical::input_output;
    use std::path::Path;

    fn test(folder: &str) {
        let (actual_output, desired_output) = generate_outputs(folder);
        assert_eq!(actual_output, desired_output);
    }

    /// Function to generate outputs
    fn generate_outputs(directory: &str) -> (String, String) {
        let input_path_str = format!("./tests/jsons/{}/input.json", directory);
        let output_path_str = format!("./tests/jsons/{}/expected.json", directory);
        let actual_output_path_str = format!("./tests/jsons/{}/observed.json", directory);

        let input_path = Path::new(&input_path_str[..]);
        let output_path = Path::new(&output_path_str[..]);
        let actual_output_path = Path::new(&actual_output_path_str[..]);

        let input: Input = input_output::get_input_from_json(input_path).unwrap();
        let desired_output: String =
            input_output::get_output_string_from_json(output_path).unwrap();

        // ONLY do this if expected is malformatted ... check that contents don't change!
        // input_output::write_to_file(output_path, &desired_output).unwrap();

        let mut calendar = Calendar::new(input.start_date, input.end_date);

        calendar.add_budgets_from(&input.goals);

        //generate and place simple goal activities
        let simple_goal_activities =
            activity_generator::generate_simple_goal_activities(&calendar, &input.goals);
        let simple_goal_activities = adjust_parent_activities(&simple_goal_activities, &input.goals);
        activity_placer::place(&mut calendar, simple_goal_activities);

        //generate and place budget goal activities
        let budget_goal_activities: Vec<Activity> =
            activity_generator::generate_budget_goal_activities(&calendar, &input.goals);
        activity_placer::place(&mut calendar, budget_goal_activities);

        calendar.log_impossible_min_day_budgets();

        let get_to_week_min_budget_activities =
            activity_generator::generate_get_to_week_min_budget_activities(&calendar, &input.goals);
        activity_placer::place(&mut calendar, get_to_week_min_budget_activities);

        calendar.log_impossible_min_week_budgets();

        let top_up_week_budget_activities =
            activity_generator::generate_top_up_week_budget_activities(&calendar, &input.goals);
        activity_placer::place(&mut calendar, top_up_week_budget_activities);

        let output = calendar.print();

        let actual_output = serde_json::to_string_pretty(&output).unwrap();

        input_output::write_to_file(actual_output_path, &actual_output).unwrap();

        (actual_output, desired_output)
    }
}
