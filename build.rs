#[cfg(not(feature = "skip-test-generation"))]
use std::io::Write;
#[cfg(not(feature = "skip-test-generation"))]
use std::path::PathBuf;

#[cfg(not(feature = "skip-test-generation"))]
fn write_test(file: &mut std::fs::File, content: &str) -> Result<(), std::io::Error> {
    writeln!(file, "{}", content)?;
    Ok(())
}

#[cfg(not(feature = "skip-test-generation"))]
fn get_test_dirs(path: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut dirs = std::fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    dirs.sort();

    Ok(dirs)
}
#[cfg(not(feature = "skip-test-generation"))]
fn main() -> Result<(), std::io::Error> {
    let out_dir = String::from("./tests/");
    let mut result = vec!["".to_string()];

    result.push(get_run_test());
    result.push(create_tests_module());

    let mut rust_tests_file = std::fs::File::create(format!("{}/rust_tests.rs", out_dir))?;
    write_test(&mut rust_tests_file, result.join("\n").trim())?;
    Ok(())
}

#[cfg(feature = "skip-test-generation")]
fn main() -> Result<(), std::io::Error> {
    Ok(())
}

#[cfg(not(feature = "skip-test-generation"))]
fn get_run_test() -> String {
    include_str!("build_templates/run_test.rs").to_string()
}

#[cfg(not(feature = "skip-test-generation"))]
fn get_test_fn_template(dir_name: &str, test_type: TestType) -> String {
    let test_name = dir_name.replace('-', "_");
    let mut test_fn_template: String = if let TestType::Experimental = test_type {
        "    #[cfg_attr(not(feature = \"experimental-testset\"), ignore)]\n".to_string()
    } else {
        String::new()
    };
    test_fn_template.push_str(include_str!("build_templates/test_fn.rs"));

    test_fn_template = test_fn_template.replace("TEST_NAME", &test_name);
    test_fn_template = test_fn_template.replace("DIR_NAME", dir_name);
    test_fn_template = test_fn_template.replace(
        "FOLDER_NAME",
        if let TestType::Experimental = test_type {
            "experimental"
        } else {
            "stable"
        },
    );

    test_fn_template
}

#[cfg(not(feature = "skip-test-generation"))]
fn create_tests_module() -> String {
    // let mut result = vec!["".to_string()];
    let module_name = "e2e";
    let mut tests_mod = include_str!("build_templates/tests_mod.rs").to_string();

    tests_mod = tests_mod.replace("TEST_MODULE_NAME", module_name);
    tests_mod = tests_mod.replace(
        "//TEST_FUNCTIONS_STABLE",
        &create_test_functions("./tests/jsons/stable", TestType::Stable),
    );
    tests_mod = tests_mod.replace(
        "//TEST_FUNCTIONS_EXPERIMENTAL",
        &create_test_functions("./tests/jsons/experimental", TestType::Experimental),
    );

    tests_mod
}

#[cfg(not(feature = "skip-test-generation"))]
fn create_test_functions(root_dir: &str, test_type: TestType) -> String {
    let mut dirs = get_test_dirs(root_dir).expect("Unable to read tests directory");
    let mut result = vec!["".to_string()];

    dirs.retain(|d| (d.is_dir()));

    for d in dirs.iter() {
        if let Ok(mut _dir) = d.read_dir() {
            result.push(get_test_fn_template(
                d.file_name().unwrap().to_str().unwrap(),
                test_type,
            ));
        }
    }

    result
        .into_iter()
        .collect::<String>()
        .trim_end()
        .to_string()
}

#[cfg(not(feature = "skip-test-generation"))]
#[derive(Clone, Copy)]
enum TestType {
    Stable,
    Experimental,
}
