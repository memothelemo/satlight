use std::{env, process, sync::Arc, fs};
use lunar::*;

fn print_error(file: &str, code: &str, err: &dyn shared::AnyAstError) {
    if let Some(err) = err.as_with_span() {
        eprintln!("{}:{}: {}", file, err.position(code), err.message(code).unwrap());
    } else {
        eprintln!("{}:{}", file, err.as_normal().unwrap().message());
    }
}

fn parse(file_name: &str, contents: &str) -> Result<ast::Block, ()> {
    use parser::Parser;
    let tokens = match tokenizer::tokenize(contents) {
        Ok(tokens) => tokens,
        Err(err) => return {
            print_error(file_name, contents, &err);
            Err(())
        },
    };
    let tokens = ast::filter_non_trivia_tokens(tokens);
    let state = parser::ParseState::new(&tokens);
    let (_, block) = match parser::ParseBlock.parse(&state) {
        Ok(block) => block,
        Err(err) => return {
            print_error(file_name, contents, &err);
            Err(())
        },
    };
    Ok(block)
}

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        eprintln!("No input files");
        process::exit(1);
    }
    args.next();

    let mut file_names = Vec::new();
    for arg in args {
        file_names.push(arg.to_string());
    }

    let options = Arc::new(config::CompilerOptions {
        multicore_typechecking: true
    });

    let file_names = Arc::new(file_names);
    rayon::scope(move |s| {
        for file_name in file_names.iter() {
            let file_name = Arc::new(file_name.to_string());
            let options = options.clone();
            s.spawn(move |_| {
                println!("Compiling: {}", &file_name);

                let contents = fs::read_to_string(file_name.as_ref()).unwrap();
                let block = match parse(&file_name, &contents) {
                    Ok(b) => b,
                    Err(_) => return,
                };

                let mut checker = typecheck::Typechecker::new(options.as_ref().clone());
                checker.bind_block(&block, None);

                macro_rules! stop_if_diags {
                    () => {
                        for err in checker.diagnostics().iter() {
                            print_error(&file_name, &contents, err);
                        }
                        if !checker.diagnostics().is_empty() {
                            return;
                        }
                    };
                }
                stop_if_diags!();
                checker.check_all();
                stop_if_diags!();
            });
        }
    });
}
