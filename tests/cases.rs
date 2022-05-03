mod common;
use common::run_test_folder;

#[test]
fn pass_project_case_test() {
    run_test_folder("./tests/project_samples/pass", |path| {
        print!("Testing project directory {}: ", path.to_string_lossy());
        lunar::env::project::from_dir(path).expect("Failed");
        println!("Passed");
    });
}

#[test]
fn fail_project_case_test() {
    run_test_folder("./tests/project_samples/fail", |path| {
        print!("Testing project directory {}: ", path.to_string_lossy());
        lunar::env::project::from_dir(path).expect_err("Failed");
        println!("Passed");
    });
}
