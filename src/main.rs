// #[macro_use]
extern crate combine;
use combine::{many,Parser};
use combine::char::letter;

fn main() {
    println!("Hello, world!");
}

#[test]
fn readme() {
    let result = many(letter()).parse("hello, world");
    assert_eq!(result, Ok(("hello".to_string(), ", world")));
}
