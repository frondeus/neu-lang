use neu_parser::core::{Lexer, State};
use neu_parser::parser;
use test_case::test_case;

/*
    TODO:
       * [x] Integers
       * [x] Immediate Constants
       * [x] Unary primitives
       * [ ] Binary primitives
       * [ ] Local variables
       * [ ] Conditional Expressions
       * [ ] Procedure Calls
       * [ ] Structs
       * [ ] Arrays
       * [ ] Strings
       * [ ] References
       * [ ] Closures
       * [ ] Heap Allocation
       * [ ] Proper tail calls
       * [ ] Complex constants
       * [ ] Assignment
       * [ ] Libraries
*/

#[test_case("4", "number")]
#[test_case("true", "bool_t")]
#[test_case("false", "bool_f")]
#[test_case("-5", "unary_int")]
#[test_case("!true", "unary_bool")]
#[test_case("    5", "skip_trivia")]
#[test_case("???", "error")]
fn tests(input: &str, test_case_name: &str) {
    {
        let lexer = Lexer::new(input);

        let res: Vec<_> = lexer.map(|t| t.display(input, true).to_string()).collect();
        neu_parser::core::testing::snap(
            format!("```\n{}\n```\n\n{:#?}", input, res),
            file!(),
            &format!("lexer_{}", test_case_name),
        );
    }
    {
        let lexer = Lexer::new(input);

        let res = State::parse(lexer, parser());
        neu_parser::core::testing::snap(
            format!("```\n{}\n```\n\n{}", input, res.display(input)),
            file!(),
            &format!("parser_{}", test_case_name),
        );
    }
}
