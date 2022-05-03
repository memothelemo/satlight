mod common;

use std::{
    fs,
    path::{Path, PathBuf},
};

use common::run_test_folder;

use lunar_checker::{analyzer::Analyzer, binder::Binder};
use lunar_common::{Config, ConfigInfo};

use lunar_parser::parse_file;
use lunar_tokenizer::tokenize;

#[test]
fn test_pass_cases() {
    run_test_folder("./tests/cases/pass", &|path| {
        print!("Testing {}", path.to_string_lossy());
        let source = fs::read_to_string(path).expect("couldn't read the script file");
        let tokens = tokenize(&source).expect("failed to tokenize");
        let tokens = lunar_ast::filter_non_trivia_tokens(tokens);
        let file = parse_file(&tokens).expect("failed to parse");

        let (binder, block) = Binder::new(&file);
        let config = Config::new(ConfigInfo::default(), PathBuf::from("."));
        let result = Analyzer::analyze(&binder, &config, &block);
        match result {
            Ok(_) => println!("Passed"),
            Err(error) => panic!("Failed: {}", error),
        }
    });
}

#[test]
fn test_fail_cases() {
    run_test_folder("./tests/cases/fail", &|path| {
        let source = fs::read_to_string(path).expect("couldn't read the script file");
        let tokens = tokenize(&source).expect("failed to tokenize");
        let tokens = lunar_ast::filter_non_trivia_tokens(tokens);
        let file = parse_file(&tokens).expect("failed to parse");

        let (binder, block) = Binder::new(&file);
        let config = Config::new(ConfigInfo::default(), PathBuf::from("."));
        let result = Analyzer::analyze(&binder, &config, &block);
        match result {
            Ok(_) => panic!("Expected fail"),
            Err(error) => {
                println!("Passed");
                let output_path = Path::new(path).with_extension("error");
                fs::write(output_path, error.to_string()).expect("failed to write");
            }
        }
    });
}
