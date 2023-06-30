use std::io::Write;
use std::path::PathBuf;

fn write_test(file: &mut std::fs::File, content: &mut str) -> Result<(), std::io::Error> {
    writeln!(file, "{}", content)?;
    Ok(())
}

fn get_test_dirs() -> Result<Vec<PathBuf>, std::io::Error> {
    let mut dirs = std::fs::read_dir("./tests/jsons")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    dirs.sort();

    Ok(dirs)
}

fn main() -> Result<(), std::io::Error> {
    let out_dir = String::from("./tests/");
    let mut result = vec!["".to_string()];

    result.push(get_run_test());
    result.push(create_tests_module());

    let mut rust_tests_file = std::fs::File::create(format!("{}/rust_tests.rs", out_dir))?;
    write_test(
        &mut rust_tests_file,
        &mut result.join("\n").trim().to_owned(),
    )?;
    Ok(())
}

fn get_run_test() -> String {
    include_str!("build_templates/run_test.rs").to_string()
}

fn get_test_fn_template(dir_name: &str, is_warn: bool) -> String {
    let test_name = dir_name.replace('-', "_");
    let mut test_fn_template: String;

    if is_warn {
        test_fn_template = include_str!("build_templates/test_fn _soft_assertion.rs").to_string();
    } else {
        test_fn_template = include_str!("build_templates/test_fn.rs").to_string();
    }

    test_fn_template = test_fn_template.replace("TEST_NAME", &test_name);
    test_fn_template = test_fn_template.replace("DIR_NAME", dir_name);

    test_fn_template
}

fn create_tests_module() -> String {
    // let mut result = vec!["".to_string()];
    let module_name = "e2e";
    let mut tests_mod = include_str!("build_templates/tests_mod.rs").to_string();

    tests_mod = tests_mod.replace("TEST_MODULE_NAME", module_name);
    tests_mod = tests_mod.replace("//TEST_FUNCTIONS", &create_test_functions());

    tests_mod
}

fn create_test_functions() -> String {
    let mut dirs = get_test_dirs().expect("Unable to read tests directory");
    let mut result = vec!["".to_string()];

    dirs.retain(|d| !d.file_name().unwrap().eq("benchmark-tests") && (d.is_dir()));

    for d in dirs.iter() {
        if let Ok(mut dir) = d.read_dir() {
            if dir.next().is_none() {
                result.push(get_test_fn_template(
                    d.file_name().unwrap().to_str().unwrap(),
                    true,
                ));
            } else {
                result.push(get_test_fn_template(
                    d.file_name().unwrap().to_str().unwrap(),
                    false,
                ));
            }
        }
    }

    result
        .into_iter()
        .collect::<String>()
        .trim_end()
        .to_string()
}
