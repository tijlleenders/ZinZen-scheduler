#[cfg(test)]
mod TEST_MODULE_NAME {

    // stable tests
    //TEST_FUNCTIONS_STABLE

    // experimental tests
    //TEST_FUNCTIONS_EXPERIMENTAL

    use crate::common;
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

        let input: Input = common::get_input_from_json(input_path).unwrap();
        let desired_output: String = common::get_output_string_from_json(output_path).unwrap();

        let output: FinalTasks = scheduler::run_scheduler(input);
        let actual_output = serde_json::to_string_pretty(&output).unwrap();

        common::write_to_file(actual_output_path, &actual_output).unwrap();

        (actual_output, desired_output)
    }
}
