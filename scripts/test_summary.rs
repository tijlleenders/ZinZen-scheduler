use std::process::Command;

fn main() {
    let mut command = Command::new("cargo");
    command.args(["test", "--no-fail-fast"]);

    let test_output = command.output().expect("failed to execute process");
    let test_output_string = String::from_utf8_lossy(&test_output.stdout);

    let final_output: String;

    match test_output_string.rfind("failures:") {
        Some(fails_index) => {
            let filtered_string = &test_output_string[fails_index..test_output_string.len()];
            final_output = sort_tests(filtered_string);
        }
        None => {
            let final_index = test_output_string.rfind("test result:").unwrap();
            final_output = format!(
                "{}",
                &test_output_string[final_index..test_output_string.len()]
            );
        }
    }
    println!("{}", final_output);
}

fn sort_tests(failed_tests: &str) -> String {
    let start_index = failed_tests.find("failures:").unwrap() + "failures:".len();
    let header = failed_tests.lines().next().unwrap().clone().trim();

    let end_index = failed_tests.find("test result:").unwrap();
    let footer = &failed_tests[end_index..failed_tests.len()].trim();

    let result = format!(
        "{}\n{}\n{}",
        header,
        &failed_tests[start_index..end_index],
        footer
    );

    result
}
