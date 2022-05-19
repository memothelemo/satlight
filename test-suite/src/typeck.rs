use super::*;
use salite::{
    checker::{Analyzer, EnvContext, Resolver},
    common::{errors::SaliteError, memory::SafePtr},
};
use std::path::Path;

pub struct TypeckCase;

pub fn run_scripts_folder<P: AsRef<Path>>(
    folder: P,
    callback: &mut impl FnMut(&Path, PathBuf),
) -> Result<(), String> {
    match std::fs::read_dir(&folder) {
        Ok(dir) => {
            for entry in dir {
                let entry = entry.unwrap();
                let path = entry.path().canonicalize().unwrap();

                // check if that file extension is slt only
                let file_ext = entry.path();
                if let Some(file_ext) = file_ext.extension() {
                    if file_ext == "slt" {
                        let display_path = path
                            .strip_prefix(TEST_REAL_PATH.clone())
                            .map_err(|e| e.to_string())?
                            .to_path_buf();

                        callback(&path, display_path);
                    }
                }

                if path.is_dir() {
                    run_scripts_folder(path, callback)?;
                }
            }
        }
        Err(..) => {
            return Err(format!(
                "couldn't read directory: {}",
                folder.as_ref().to_string_lossy()
            ))
        }
    }
    Ok(())
}

impl TypeckCase {
    pub fn sample_path(&self, env: &TestEnv) -> PathBuf {
        env.sample_path().join("typeckr")
    }

    pub fn fail_path(&self, env: &TestEnv) -> PathBuf {
        self.sample_path(env).join("fail")
    }

    pub fn pass_path(&self, env: &TestEnv) -> PathBuf {
        self.sample_path(env).join("pass")
    }

    pub fn deal_error<T>(&self, result: Result<T, String>, env: &mut TestEnv) {
        if let Err(err) = &result {
            env.fail(err);
        }
    }

    pub fn parse_script(&self, path: &Path) -> Result<salite::ast::File, String> {
        let source = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let file = match salite::lazy_parse(false, &source) {
            Ok(file) => file,
            Err(err) => {
                return Err(match err.message(&source).map_err(|e| e.to_string()) {
                    Ok(err) => err,
                    Err(err) => err,
                })
            }
        };
        Ok(file)
    }

    pub fn evaluate_script(&self, path: &Path) -> Result<(), String> {
        let file = self.parse_script(path)?;
        let cfg = salite::common::Config::default();

        let mut env_ctx = EnvContext::new(&cfg);
        let env_ptr = SafePtr::from_ptr(&mut env_ctx as *mut EnvContext);

        env_ctx.add_module(path.to_path_buf(), &file);

        let result = env_ctx.get_module_result_mut(&path.to_path_buf()).unwrap();

        Resolver::from_result(result, env_ptr).map_err(|e| e.to_string())?;
        Analyzer::analyze(result.ctx.clone(), &result.file).map_err(|e| e.to_string())?;

        Ok(())
    }
}

impl TestCase for TypeckCase {
    fn name(&self) -> &'static str {
        "typechecker"
    }

    fn on_run(&self, env: &mut TestEnv) {
        env.describe("pass cases");
        self.deal_error(
            run_scripts_folder(self.pass_path(env), &mut |file, buf| {
                let result = match self.evaluate_script(file) {
                    Ok(..) => Ok(()),
                    Err(err) => Err(err),
                };
                env.it(
                    format!("should pass on file {}", buf.to_string_lossy()).as_str(),
                    || result.clone(),
                );
                drop(result);
            }),
            env,
        );
        env.describe_end();
        env.describe("fail cases");
        self.deal_error(
            run_scripts_folder(self.fail_path(env), &mut |file, buf| {
                let result = match self.evaluate_script(file) {
                    Ok(..) => Err("Expected fail".to_string()),
                    #[cfg(feature = "no-out")]
                    Err(..) => Ok(()),
                    #[cfg(not(feature = "no-out"))]
                    Err(err) => {
                        let output_path = Path::new(file).with_extension("error");
                        std::fs::write(output_path, err).expect("failed to write");
                        Ok(())
                    }
                };
                env.it(
                    format!("should fail on file {}", buf.to_string_lossy()).as_str(),
                    || result.clone(),
                );
                drop(result);
            }),
            env,
        );
        env.describe_end();
    }
}
