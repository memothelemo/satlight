use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lunar_tokenizer::tokenize as tokenizer;

const TC_SOURCE: &str = include_str!("./typecheck.lun");

fn check(criterion: &mut Criterion) {
    use lunar_parser::Parser;
    let tokens = lunar_ast::filter_non_trivia_tokens(tokenizer(TC_SOURCE).unwrap());
    let state = lunar_parser::ParseState::new(&tokens);
    let (_, block) = lunar_parser::ParseBlock.parse(&state).unwrap();

    criterion.bench_function("check typecheck.lr", move |b| {
        b.iter(|| {
            // preparing for typechecking
            let mut typechecker = black_box({
                let mut checker = lunar_typecheck::Typechecker::new();
                checker.preload_enviroment(&mut lunar_typecheck::LunarStandardProvider);
                checker
            });
            typechecker.visit_block(&block)
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(20);
    targets = check
}

criterion_main!(benches);
