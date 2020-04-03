#![allow(dead_code)]
use parsing_tutorial::input::Input;
use parsing_tutorial::lexer::{Lex, Lexer, OptionExt};
use parsing_tutorial::node_builder::NodeBuilder;
use parsing_tutorial::parser::{Parser, Context};
use parsing_tutorial::parsers::{node, token, trivia, v_node, ParserExt};
use parsing_tutorial::token::TokenKind;
use parsing_tutorial::node::{Name, Nodes};

#[derive(Debug, PartialEq)]
pub enum Token {
    Error,
    Atom,
    Trivia, // Whitespace
    OpenParent,
    CloseParent,
}

impl TokenKind for Token {
    fn is_error(&self) -> bool {
        match self {
            Self::Error => true,
            _ => false,
        }
    }
}

pub fn lexer(input: &str) -> impl Lexer<Token> {
    Lex::new(input, |i: &mut Input| {
        Some(match i.as_ref().chars().next()? {
            c if c.is_whitespace() => {
                let rest = i.as_ref().chars().take_while(|c| c.is_whitespace()).count();
                (Token::Trivia, i.chomp(rest))
            }
            c if c.is_alphanumeric() => {
                let rest = i
                    .as_ref()
                    .chars()
                    .take_while(|c| c.is_alphanumeric())
                    .count();
                (Token::Atom, i.chomp(rest))
            }
            '(' => (Token::OpenParent, i.chomp(1)),
            ')' => (Token::CloseParent, i.chomp(1)),
            _ => (Token::Error, i.chomp(1)),
        })
    })
}

fn skip_trivia<Lex>(builder: &mut NodeBuilder, context: &mut Context<Lex>)
where Lex: Lexer<Token>
{
    loop {
        match context.lexer_mut().peek().map(|t| &t.kind) {
            Some(Token::Trivia) => {
                builder.parse(trivia(), context);
            }
            _ => {
                return;
            }
        }
    }
}

struct Value;
#[allow(non_upper_case_globals)]
impl Value {
    const SExp: Name = Name::new("SExp");
    const List: Name = Name::new("List");
    const Nil: Name = Name::new("Nil");
    const Atom: Name = Name::new("Atom");
    const Trivia: Name = Name::new("Trivia");
}

fn parser<Lex>() -> impl Parser<Lex, Token>
where
    Lex: Lexer<Token>,
{
    parse_sexp()
}

fn parse_sexp<Lex>() -> impl Parser<Lex, Token>
where
    Lex: Lexer<Token>,
{
    v_node(|builder, context: &mut Context<Lex>| {
        builder.name(Value::SExp);
        skip_trivia(builder, context);
        let peeked = context.lexer_mut().peek();
        match peeked.as_kind() {
            None => {
                builder.parse(
                    node(|builder, context: &mut Context<Lex>| {
                        builder.name(Nodes::Error);
                        context.lexer_mut().next();
                        context.error("Expected SExp found EOF");
                    }),
                    context);
            },
            Some(Token::Atom) => builder.parse(
                token(Token::Atom).map(|node| node.with_name(Value::Atom)),
                context,
            ),
            Some(Token::OpenParent) => builder.parse(parse_list(), context),
            Some(Token::Trivia) => unreachable!(),
            Some(_) => {
                builder.parse(
                    node(|builder, context: &mut Context<Lex>| {
                        builder.name(Nodes::Error);
                        let token = context.lexer_mut().next();
                        context.error(format!("Expected SExp found {:?}", token.as_kind().unwrap()));
                    }),
                    context);
            },
        };
        skip_trivia(builder, context);
    })
}

fn parse_list<Lex>() -> impl Parser<Lex, Token>
where
    Lex: Lexer<Token>,
{
    node(|builder, context: &mut Context<Lex>| {
        skip_trivia(builder, context);
        builder.parse(token(Token::OpenParent), context);
        skip_trivia(builder, context);
        let peeked = context.lexer_mut().peek();
        match peeked.as_kind() {
            None => {
                context.error("Expected `)` or atom but found EOF");
            },
            Some(Token::CloseParent) => {
                builder.name(Value::Nil);
            }
            _ => loop {
                builder.name(Value::List);
                builder.parse(parse_sexp(), context);
                let peeked = context.lexer_mut().peek();
                match peeked.as_kind() {
                    None => {
                        break;
                    },
                    Some(Token::CloseParent) => break,
                    _ => continue,
                }
            },
        }
        builder.parse(token(Token::CloseParent), context);
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("(add 2 (京 4 5))", "unicode" ; "unicode")]
    #[test_case("(add 2 (+++ 4 5))", "error" ; "error")]
    fn lexer_tests(input: &str, test_case_name: &str) {
        let lexer = lexer(input);

        let res: Vec<_> = lexer.map(|t| t.display(input).to_string()).collect();
        parsing_tutorial::testing::snap(
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
        let lexer = lexer(input);

        let res = Context::parse(lexer, parser());
        parsing_tutorial::testing::snap(
            format!("```\n{}\n```\n\n{:#?}", input, res),
            file!(),
            &format!("parser_{}", test_case_name),
        );
    }
}

fn main() {}
