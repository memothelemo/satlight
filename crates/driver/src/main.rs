use lunar::*;

static CODE: &str = include_str!("sample.lun");

fn print_error(err: &dyn shared::AnyAstError) {
    if let Some(err) = err.as_with_span() {
        eprintln!("{}: {}", err.position(CODE), err.message(CODE).unwrap());
    } else {
        eprintln!("{}", err.as_normal().unwrap().message());
    }
}

fn main() {
    let tokens = match tokenizer::tokenize(CODE) {
        Ok(tokens) => tokens,
        Err(err) => return print_error(&err),
    };
    use parser::Parser;

    let tokens = ast::filter_non_trivia_tokens(tokens);
    let state = parser::ParseState::new(&tokens);
    let (_, block) = match parser::ParseBlock.parse(&state) {
        Ok(block) => block,
        Err(err) => return print_error(&err),
    };

    #[rustfmt::skip]
    let mut checker = typecheck::Typechecker::new(
        config::CompilerOptions {
            multicore_typechecking: true,
        },
    );
    checker.bind_block(&block, None);

    macro_rules! stop_if_diags {
        () => {
            for err in checker.diagnostics().iter() {
                print_error(err);
            }
            if !checker.diagnostics().is_empty() {
                return;
            }
        };
    }

    stop_if_diags!();
    dbg!(&checker);
    checker.check_all();
    stop_if_diags!();

    // let mut typechecker = typecheck::Typechecker::new();
    // typechecker.preload_enviroment(&mut typecheck::LunarStandardProvider);

    // let hir_block = typechecker.visit_block(&block);
    // println!("{:#?}", hir_block);
    // println!("{:#?}", typechecker);

    // for diag in typechecker.diagnostics().iter() {
    //     print_error(diag);
    // }

    // let mut hir_env = hir::HirEnvironment::new();
    // hir_env.load_file(&LunarStandard, "stdin".to_string(), &block);

    // let ty_config = typechecker::TypecheckConfig {
    //     enable_multi_threading: true,
    // };

    // let mut typechecker = typechecker::Typechecker::new(ty_config, hir_env);
    // typechecker.check_all();
}
