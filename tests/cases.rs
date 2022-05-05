mod common;
use common::run_test_folder;

#[test]
fn pass_project_case_test() {
    run_test_folder("./tests/project_samples/pass", |path| {
        salite::env::project::from_dir(path).expect("Failed");
    });
}

#[test]
fn fail_project_case_test() {
    run_test_folder("./tests/project_samples/fail", |path| {
        salite::env::project::from_dir(path).expect_err("Failed");
    });
}
