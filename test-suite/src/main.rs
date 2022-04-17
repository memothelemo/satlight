use lunar_ast as ast;
use lunar_parser as parser;
use lunar_tokenizer as tokenizer;

use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

macro_rules! create_parse_case {
    {
        uses = $uses:expr,
        opposite_name = $op_fn:ident,
        name = $fn:ident,
        dir_reader = $dir:ident,
    } => {
        fn $op_fn(path: &Path) {
            use parser::Parser;
            println!("Testing (diag): {}", path.to_string_lossy());

            let file_contents = fs::read_to_string(path).unwrap();
            let tokens = tokenizer::tokenize(&file_contents)
                .unwrap_or_else(|e| panic!("Failed to tokenize {}: {:#?}", path.to_string_lossy(), e));

            let tokens = ast::filter_non_trivia_tokens(tokens);

            let state = parser::ParseState::new(&tokens);
            let err = $uses
                .parse(&state)
                .expect_err("Successfully parsed");

            let output_path = Path::new(path).with_extension("result");
            let output = serde_json::to_string_pretty(&err).unwrap();

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

        fn $fn(path: &Path) {
            use parser::Parser;
            println!("Testing (ok): {}", path.to_string_lossy());

            let file_contents = fs::read_to_string(path).unwrap();
            let tokens = tokenizer::tokenize(&file_contents)
                .unwrap_or_else(|e| panic!("Failed to tokenize {}: {:#?}", path.to_string_lossy(), e));

            let tokens = ast::filter_non_trivia_tokens(tokens);

            let state = parser::ParseState::new(&tokens);
            let (_, result) = $uses
                .parse(&state)
                .unwrap_or_else(|e| panic!("Failed to parse {}: {:#?}", path.to_string_lossy(), e));

            let output_path = Path::new(path).with_extension("result");
            let output = serde_json::to_string_pretty(&result).unwrap();

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

        fn $dir(path: &Path, opposite: bool) {
            let result = fs::read_dir(path)
                .unwrap_or_else(|e| panic!("Failed to read directory {}: {}", path.to_string_lossy(), e));

            for file in result {
                let file_path = file.unwrap();
                if file_path.file_type().unwrap().is_file() {
                    let file_ext = file_path.path();
                    let file_ext = file_ext.extension().unwrap();
                    if file_ext == "cl" {
                        let finalized_path = file_path.path();
                        let finalized_path = finalized_path.as_path();
                        if opposite {
                            $op_fn(finalized_path);
                        } else {
                            $fn(finalized_path);
                        }
                    }
                } else {
                    $dir(file_path.path().as_path(), opposite);
                }
            }
        }
    };
}

create_parse_case! {
    uses = parser::ParseStmt,
    opposite_name = run_stmt_case_opposite,
    name = run_stmt_case,
    dir_reader = run_stmt_cases_dir,
}

create_parse_case! {
    uses = parser::ParseExpr,
    opposite_name = run_expr_case_opposite,
    name = run_expr_case,
    dir_reader = run_expr_cases_dir,
}

create_parse_case! {
    uses = parser::ParseBlock,
    opposite_name = run_block_case_opposite,
    name = run_block_case,
    dir_reader = run_block_cases_dir,
}

create_parse_case! {
    uses = parser::ParseTypeInfo,
    opposite_name = run_ty_case_opposite,
    name = run_ty_case,
    dir_reader = run_ty_cases_dir,
}

fn main() {
    run_expr_cases_dir(
        Path::new("./test-suite/parser/diagnostics/expressions"),
        true,
    );
    run_stmt_cases_dir(
        Path::new("./test-suite/parser/diagnostics/statements"),
        true,
    );
    run_block_cases_dir(Path::new("./test-suite/parser/diagnostics/block"), true);
    run_expr_cases_dir(Path::new("./test-suite/parser/expressions"), false);
    run_stmt_cases_dir(Path::new("./test-suite/parser/statements"), false);
    run_ty_cases_dir(Path::new("./test-suite/parser/types"), false);
    #[cfg(feature = "scripts")]
    run_block_cases_dir(Path::new("./test-suite/parser/scripts"), false);
}
