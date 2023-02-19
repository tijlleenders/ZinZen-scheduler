use std::io::Write;
use std::path::PathBuf;

fn write_test(file: &mut std::fs::File, content: &mut str) -> Result<(), std::io::Error> {
    write!(file, "{}\n", content)?;
    Ok(())
}

fn get_test_dirs() -> Result<Vec<PathBuf>, std::io::Error> {
    let dirs = std::fs::read_dir("./tests/jsons")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;
    Ok(dirs)
}

fn test_template(test_name: &str, test_dir: &str) -> String {
    let mut result = vec!["\n#[test]\n".to_string()];
    result.push(format!("fn {}() {{", test_name));
    result.push(format!(
        "\n{}let (actual_output, desired_output) = run_test(\"{}\");",
        space(),
        test_dir
    ));
    result.push(format!(
        "\n{}assert_eq!(actual_output, desired_output);\n}}",
        space()
    ));
    result.join("")
}
fn space() -> String {
    "    ".to_string()
}
fn get_imports() -> String {
    let mut result = vec!["extern crate scheduler;".to_string()];
    result.push("\nmod common;".to_string());
    result.push("\n#[cfg(test)]".to_string());
    result.push("\nuse pretty_assertions::assert_eq;".to_string());
    result.push("\nuse scheduler::{FinalOutput, Input};".to_string());
    result.push("\nuse std::path::Path;\n\n".to_string());
    result.join("")
}

fn main() -> Result<(), std::io::Error> {
    let out_dir = String::from("./tests/");
    let mut dirs = get_test_dirs().expect("Unable to read tests directory");
    dirs.retain(|d| !d.file_name().unwrap().eq("benchmark-tests") && (d.is_dir()));

    let dirs_vec = dirs
        .iter()
        .map(|d| d.file_name().unwrap().to_str().unwrap())
        .collect::<Vec<_>>();

    let mut result = vec!["".to_string()];
    result.push(get_imports());
    result.push(get_run_test());
    for dir in dirs_vec.iter() {
        result.push(test_template(dir.replace("-", "_").as_str(), dir));
    }

    let mut rust_tests_file = std::fs::File::create(&format!("{}/rust_tests.rs", out_dir))?;
    write_test(&mut rust_tests_file, &mut result.join(""))?;
    Ok(())
}

fn get_run_test() -> String {
    "fn run_test(directory: &str) -> (String, String) {
    let i = format!(\"./tests/jsons/{}/input.json\", directory);
    let o = format!(\"./tests/jsons/{}/output.json\", directory);
    let input_path = Path::new(&i[..]);
    let output_path = Path::new(&o[..]);
    let input: Input = common::get_input_from_json(input_path).unwrap();
    let desired_output: String = common::get_output_string_from_json(output_path).unwrap();
    let output: FinalOutput = scheduler::run_scheduler(input);
    (
        serde_json::to_string_pretty(&output).unwrap(),
        desired_output,
    )
}"
    .to_string()
}
