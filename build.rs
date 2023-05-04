use std::io::Write;
use std::path::PathBuf;

fn write_test(file: &mut std::fs::File, content: &mut str) -> Result<(), std::io::Error> {
    writeln!(file, "{}", content)?;
    Ok(())
}

fn get_test_dirs() -> Result<Vec<PathBuf>, std::io::Error> {
    let dirs = std::fs::read_dir("./tests/jsons")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;
    dirs.sort();
    Ok(dirs)
}

fn test_template(test_dir: &str, is_warn: bool) -> String {
    let test_name = test_dir.replace('-', "_");
    let mut result = vec!["\n#[test]\n".to_string()];
    result.push(format!("fn {}() {{", test_name));
    if is_warn {
        result.push(format!("\n{}SimpleLogger::new().init().unwrap();", space()));
        result.push(format!(
            "\n{}log::warn!(\n{0}{0}\"Empty directory Or one of input.json & output.json not exist: {{}}\",\n{0}{0}\"{:}\"\n{0});",
            space(),
            test_dir
        ));
    }
    result.push(format!(
        "\n{}let (actual_output, desired_output) = run_test(\"{}\");",
        space(),
        test_dir
    ));
    if is_warn {
        result.push(format!(
            "\n{}soft::assert_eq!(actual_output, desired_output).unwrap();\n}}",
            space()
        ));
    } else {
        result.push(format!(
            "\n{}assert_eq!(actual_output, desired_output);\n}}",
            space()
        ));
    }
    result.join("")
}
fn space() -> String {
    "    ".to_string()
}
fn get_imports() -> String {
    let mut result = vec!["extern crate scheduler;".to_string()];

    result.push("\nextern crate soft;".to_string());
    result.push("\nmod common;".to_string());
    result.push("\n#[cfg(test)]".to_string());
    result.push("\nuse pretty_assertions::assert_eq;".to_string());
    result.push("\nuse scheduler::models::{input::Input, output::FinalOutput};".to_string());
    result.push("\nuse std::path::Path;".to_string());

    result.join("")
}

fn main() -> Result<(), std::io::Error> {
    let out_dir = String::from("./tests/");
    let mut dirs = get_test_dirs().expect("Unable to read tests directory");
    let mut result = vec!["".to_string()];
    result.push(get_imports());
    result.push(get_run_test());
    dirs.retain(|d| !d.file_name().unwrap().eq("benchmark-tests") && (d.is_dir()));

    for d in dirs.iter() {
        if let Ok(mut dir) = d.read_dir() {
            if dir.next().is_none() {
                result.push(test_template(
                    d.file_name().unwrap().to_str().unwrap(),
                    true,
                ))
            } else {
                result.push(test_template(
                    d.file_name().unwrap().to_str().unwrap(),
                    false,
                ));
            }
        }
    }

    let mut rust_tests_file = std::fs::File::create(format!("{}/rust_tests.rs", out_dir))?;
    write_test(
        &mut rust_tests_file,
        &mut result.join("\n").trim().to_owned(),
    )?;
    Ok(())
}

fn get_run_test() -> String {
    "
fn run_test(directory: &str) -> (String, String) {
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
