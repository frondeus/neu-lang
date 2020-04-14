use neu_parser::core::{Lexer, State};
use neu_parser::parser;

/*
    TODO:
       * [x] Integers
       * [x] Immediate Constants
       * [x] Unary primitives
       * [x] Binary primitives
       * [x] Pratt parsers
       * [x] ConstStrings
       * [x] Parens
       * [x] Eval
       * [x] Better snapshots
       * [ ] Proper pratt span
       * [ ] Structs
       * [ ] Arrays
       * [ ] Local variables
       * [ ] Conditional Expressions
       * [ ] Procedure Calls
       * [ ] References
       * [ ] Closures
       * [ ] Heap Allocation
       * [ ] Proper tail calls
       * [ ] Complex constants
       * [ ] Assignment
       * [ ] Libraries
*/

#[test]
fn lexer_tests() {
    test_runner::test_snapshots("lexer", |input| {
        let lexer = Lexer::new(input);

        let res: Vec<_> = lexer.map(|t| t.display(input, true).to_string()).collect();
        format!("{:#?}", res)
    }).unwrap();
}

#[test]
fn parser_tests() {
    test_runner::test_snapshots("parser", |input| {
        let lexer = Lexer::new(input);

        let res = State::parse(lexer, parser());

        format!("{}", res.display(input))
    }).unwrap();
}
