use neu_parser::core::{Lexer, State};
use neu_parser::parser;
use test_case::test_case;

#[test_case("(add 2 (京 4 5))", "unicode" ; "unicode")]
#[test_case("(add 2 (+++ 4 5))", "error" ; "error")]
fn lexer_tests(input: &str, test_case_name: &str) {
    let lexer = Lexer::new(input);

    let res: Vec<_> = lexer.map(|t| t.display(input).to_string()).collect();
    neu_parser::core::testing::snap(
        format!("```\n{}\n```\n\n{:#?}", input, res),
        file!(),
        &format!("lexer_{}", test_case_name),
    );
}

#[test_case("a", "atom" ; "atom")]
#[test_case("京", "unicode_atom" ; "unicode_atom")]
#[test_case("()", "nil" ; "nil")]
#[test_case("(", "nil_error" ; "nil_error")]
#[test_case("(add 2 3)", "list" ; "list")]
#[test_case("(add ( 2 3)", "semantic_error" ; "semantic_error")]
#[test_case("(add 2 (京 4 5))", "unicode" ; "unicode")]
#[test_case("(add 2 (+++ 4 5))", "syntax_error" ; "syntax_error")]
fn parser_tests(input: &str, test_case_name: &str) {
    let lexer = Lexer::new(input);

    let res = State::parse(lexer, parser());
    neu_parser::core::testing::snap(
        format!("```\n{}\n```\n\n{}", input, res.display(input)),
        file!(),
        &format!("parser_{}", test_case_name),
    );
}
