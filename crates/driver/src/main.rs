mod prelude;
use prelude::*;

static CODE: &str = include_str!("sample.lr");

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

    // let mut hir_env = hir::HirEnvironment::new();
    // hir_env.load_file(&LunarStandard, "stdin".to_string(), &block);

    // let ty_config = typechecker::TypecheckConfig {
    //     enable_multi_threading: true,
    // };

    // let mut typechecker = typechecker::Typechecker::new(ty_config, hir_env);
    // typechecker.check_all();

    // for err in typechecker.errors.iter() {
    //     print_error(err.borrow());
    // }
}
