use std::path::Path;

use super::*;

pub struct ProjectCase;

impl ProjectCase {
    pub fn project_sample_path(&self, env: &TestEnv) -> PathBuf {
        env.sample_path().join("projects")
    }

    pub fn expect_load_fail<T: AsRef<Path>>(&self, project_dir: T) -> TestResult {
        if salite::env::project::from_dir(project_dir).is_ok() {
            Err("Expected load failed!".to_string())
        } else {
            Ok(())
        }
    }

    pub fn expect_load_success<T: AsRef<Path>>(&self, project_dir: T) -> TestResult {
        salite::env::project::from_dir(project_dir).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl TestCase for ProjectCase {
    fn name(&self) -> &'static str {
        "project_loader"
    }

    fn on_run(&self, env: &mut TestEnv) {
        env.describe("project load");

        let sample_path = self.project_sample_path(env);
        macro_rules! it_should {
            (fail $text:expr, $path_prefix:expr) => {
                env.it($text, || {
                    self.expect_load_fail(sample_path.join($path_prefix))
                });
            };
            (pass $text:expr, $path_prefix:expr) => {
                env.it($text, || {
                    self.expect_load_success(sample_path.join($path_prefix))
                });
            };
        }

        it_should!(fail "should fail if it has no config file", "no_config");
        it_should!(fail "should fail if its config failed to parse", "cfg_parse_fail");
        it_should!(pass "should pass if it pass its requirements", "sample");
    }
}
