use logos::Logos;
use derive_more::Display;
use microtree::{Ast, Red};
use microtree_parser::*;
use microtree_parser::parsers::*;

mod generated;
use generated::*;

#[derive(Logos, Debug, PartialEq, Clone, Copy, Display)]
enum Token {
    #[display("number")]
    #[regex("[0-9]+")]
    Number,

    #[display("`(`")]
    #[token("(")]
    OpenP,

    #[display("`)`")]
    #[token(")")]
    CloseP,

    #[display("`-`")]
    #[token("-")]
    OpMinus,

    #[display("`+`")]
    #[token("+")]
    OpPlus,

    #[display("`*`")]
    #[token("*")]
    OpStar,

    #[display("`/`")]
    #[token("/")]
    OpSlash,

    #[regex(r"(\r?\n)+")]
    LineEnd,

    #[regex("[ \t]+")]
    Whitespace,

    #[error]
    Error
}

impl TokenKind<'_> for Token {
    fn mergeable(self, other: Self) -> bool {
        self == Self::Error && self == other
    }
}

type Lexer<'s, T = Token> = microtree_parser::Lexer<'s, T>;

fn leading_trivia<'s>() -> impl Trivia<'s, Token> {
    |lexer: &mut Lexer<'s>| {
        dbg!(&lexer.peek());
        while let Some(Token::LineEnd) = lexer.peek_token() {
            lexer.next();
        }
        while let Some(Token::Whitespace) = lexer.peek_token() {
            lexer.next();
        }
    }
}
fn trailing_trivia<'s>() -> impl Trivia<'s, Token> {
    |lexer: &mut Lexer<'s>| loop {
        match lexer.peek_token() {
            Some(Token::Whitespace) => {lexer.next();},
            _ => break,
        }
    }
}

fn left_value<'s>() -> impl Parser<'s, Token> + Clone {
    node(|builder| {
         let mut builder = builder.name(Nodes::Value);
         match builder.peek_token() {
             Some(Token::Number) => builder.parse(number()),
             Some(Token::OpMinus) => builder.parse(unary()),
             Some(Token::OpenP) => {
                 builder.node()
                     .parse(token(Token::OpenP))
                     .parse(value())
                     .parse(token(Token::CloseP))
                     .finish()
             },
             _ => builder.parse(tokens(vec! [
                 Token::Number,
                 Token::OpMinus,
                 Token::OpenP
             ]))
         }})
}

fn number<'s>() -> impl Parser<'s, Token> {
    node(|builder| builder.name(Nodes::Number).token())
}

fn unary<'s>() -> impl Parser<'s, Token> {
    node(|builder| {
        builder.name(Nodes::Unary)
            .node()
            .parse(token(Token::OpMinus))
            .parse(value())
            .finish()
    })
}

fn value<'s>() -> impl Parser<'s, Token> {
    Pratt::new(
        left_value(),
        |token| match token {
            Some(Token::OpStar) => Some((Assoc::Left, 20)),
            Some(Token::OpSlash) => Some((Assoc::Left, 20)),

            Some(Token::OpMinus) => Some((Assoc::Left, 10)),
            Some(Token::OpPlus) => Some((Assoc::Left, 10)),

            _ => None,
        },
        |builder, op_token| {
            builder.name(Nodes::Value)
            .name(Nodes::Binary)
            .parse(named(token(op_token), Nodes::Op))
        },
    )
    .parser()
}

fn root<'s, >() -> impl Parser<'s, Token> {
    node(|builder| {
        let leading_trivia = leading_trivia();
        let trailing_trivia = trailing_trivia();
        let ctx = Context { leading_trivia: Some(&leading_trivia), trailing_trivia: Some(&trailing_trivia) };
        builder.set_ctx(&ctx)
            .parse(value())
    })
}

fn main() {
    fn act(input: &str) -> Option<Value> {
        let lexer = Lexer::new(input);
        let parsed = State::parse(lexer, root());

        dbg!(&parsed.errors);

        Value::new(Red::root(parsed.root?))
    }

    dbg!(&act("2 + 3"));
    dbg!(&act("2 + 3 * 4"));
    dbg!(&act("(2 + 3) * 4"));
    dbg!(&act("(2 + 3) \n* 4"));
}
