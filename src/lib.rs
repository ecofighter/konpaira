extern crate combine;

use combine::*;
use combine::primitives::Stream;
use combine::primitives::ParseResult;
use std::marker::PhantomData;

#[derive(Debug,Eq,PartialEq,Clone)]
pub enum AST {
    Nat (u32),
    Add (Box<AST>, Box<AST>),
    Sub (Box<AST>, Box<AST>),
    Mul (Box<AST>, Box<AST>),
    Div (Box<AST>, Box<AST>)
}

struct P<I>(PhantomData<fn(I) -> I>);

impl<I> P<I> where I: Stream<Item=char> {
    pub fn integer(input: I) -> ParseResult<AST, I> {
        many1::<String, _>(char::digit())
            .and_then(|s: String|
                      match s.parse::<u32>() {
                          Ok(num) => Ok(AST::Nat(num)),
                          Err(e) => Err(e),
                      })
            .parse_stream(input)
    }

    pub fn factor(input: I) -> ParseResult<AST, I> {
        between(token('('), token(')'), parser(P::<I>::expr))
            .or(parser(P::<I>::integer))
            .parse_stream(input)
    }

    pub fn multiple(input: I) -> ParseResult<AST, I> {
        let op =
            token('*')
            .or(token('/'))
            .map(|c|
                 move |l, r| match c {
                     '*' => AST::Mul( Box::new(l), Box::new(r) ),
                     '/' => AST::Div( Box::new(l), Box::new(r) ),
                     _ => unreachable!()
                 });
        chainl1(parser(P::<I>::factor), op).parse_stream(input)
    }

    pub fn plus(input: I) -> ParseResult<AST, I> {
        let op =
            token('+')
            .or(token('-'))
            .map(|c|
                 move |l, r| match c {
                     '+' => AST::Add( Box::new(l), Box::new(r) ),
                     '-' => AST::Sub( Box::new(l), Box::new(r) ),
                     _ => unreachable!()
                 });
        chainl1(parser(P::<I>::multiple), op).parse_stream(input)
    }

    pub fn expr(input: I) -> ParseResult<AST, I> {
        parser(P::<I>::plus).parse_stream(input)
    }
}

pub fn integer(input: &str) -> Result<(AST, &str), ParseError<&str>> {
    parser(P::integer).parse(input)
}

// pub fn expr(input: &str) -> Result<(AST, &str), ParseError<&str>> {
//     parser(P::expr).parse(input)
// }
pub fn expr<I>(input: I) -> Result<(AST, I), ParseError<I>> where I: Stream<Item=char>{
    parser(P::<I>::expr).skip(eof()).parse(input)
}
