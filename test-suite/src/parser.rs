use super::*;
use crate::typeck::run_scripts_folder;
use salite::common::errors::SaliteError;
use std::path::Path;

pub struct ParserCase;

impl ParserCase {
    pub fn deal_error<T>(&self, result: Result<T, String>, env: &mut TestEnv) {
        if let Err(err) = &result {
            env.fail(err);
        }
    }

    pub fn tokenize(&self, path: &Path) -> Result<(Vec<salite::ast::Token>, String), String> {
        let input = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let tokens = match salite::tokenizer::tokenize(&input) {
            Ok(res) => res,
            Err(err) => {
                return Err(match err.message(&input).map_err(|e| e.to_string()) {
                    Ok(err) => err,
                    Err(err) => err,
                })
            }
        };
        let tokens = salite::ast::filter_non_trivia_tokens(tokens);
        Ok((tokens, input))
    }
}

impl TestCase for ParserCase {
    fn name(&self) -> &'static str {
        "parser"
    }

    fn on_run(&self, env: &mut TestEnv) {
        macro_rules! join {
            ($adder:expr) => {
                TEST_REAL_PATH.clone().join("parser").join($adder)
            };
        }

        macro_rules! result_mattering {
            (pass = $expr:expr, $input:expr) => {
                match $expr {
                    Ok(res) => res,
                    Err(err) => {
                        return Err(match err.message(&$input).map_err(|e| e.to_string()) {
                            Ok(err) => err,
                            Err(err) => err,
                        })
                    }
                }
            };
            (fail = $expr:expr) => {
                match $expr {
                    Ok(..) => return Err("Expected fail".to_string()),
                    Err(err) => err,
                }
            };
        }

        macro_rules! parser_boilers {
            (pass = $name:ident, $location:expr, $ty:ty) => {
                env.describe($location);
                self.deal_error(
                    run_scripts_folder(join!($location), &mut |file, buf| {
                        let result: Result<$ty, String> = (|| {
                            let (tokens, input) = self.tokenize(file)?;
                            let state = salite::parser::ParseState::new(&tokens);
                            let (_, result) = result_mattering!(
                                pass = salite::parser::$name.parse(&state),
                                input
                            );

                            #[cfg(not(feature = "no-out"))]
                            {
                                let output_path = Path::new(file).with_extension("result");
                                let output = serde_json::to_string_pretty(&result).unwrap();

                                use std::{fs::File, io::Write};
                                File::create(output_path.clone())
                                    .map(|mut v| v.write_all(output.as_bytes()))
                                    .unwrap_or_else(|e| {
                                        panic!(
                                            "Failed to create output file {}: {}",
                                            output_path.to_string_lossy(),
                                            e
                                        )
                                    })
                                    .unwrap();
                            }

                            Ok(result)
                        })();
                        env.it(
                            format!("should pass on file {}", buf.to_string_lossy()).as_str(),
                            || result.clone().map(|_| ()),
                        );

                        drop(result);
                    }),
                    env,
                );
                env.describe_end();
            };
            (fail = $name:ident, $location:expr) => {
                env.describe($location);
                self.deal_error(
                    run_scripts_folder(join!($location), &mut |file, buf| {
                        let result: Result<(), String> = (|| {
                            let (tokens, ..) = self.tokenize(file)?;
                            let state = salite::parser::ParseState::new(&tokens);
                            #[cfg(feature = "no-out")]
                            result_mattering!(fail = salite::parser::$name.parse(&state));

                            #[cfg(not(feature = "no-out"))]
                            {
                                let err =
                                    result_mattering!(fail = salite::parser::$name.parse(&state));
                                let output_path = Path::new(file).with_extension("result");
                                let output = serde_json::to_string_pretty(&err).unwrap();

                                use std::{fs::File, io::Write};
                                File::create(output_path.clone())
                                    .map(|mut v| v.write_all(output.as_bytes()))
                                    .unwrap_or_else(|e| {
                                        panic!(
                                            "Failed to create output file {}: {}",
                                            output_path.to_string_lossy(),
                                            e
                                        )
                                    })
                                    .unwrap();
                            }

                            Ok(())
                        })();
                        env.it(
                            format!("should fail on file {}", buf.to_string_lossy()).as_str(),
                            || result.clone(),
                        );
                        drop(result);
                    }),
                    env,
                );
                env.describe_end();
            };
        }

        use salite::ast;
        use salite::parser::Parser;

        parser_boilers!(pass = ParseExpr, "expressions", ast::Expr);
        parser_boilers!(pass = ParseStmt, "statements", ast::Stmt);
        parser_boilers!(pass = ParseBlock, "scripts", ast::Block);
        parser_boilers!(pass = ParseTypeInfo, "types", ast::TypeInfo);
        parser_boilers!(fail = ParseBlock, "confusables");
    }
}
