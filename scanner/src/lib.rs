pub mod token;
mod scanner;

use crate::scanner::Scanner;
use crate::token::Token;

pub fn make(source: &String) -> impl Iterator<Item = Token> {
    Scanner {
        source: source.as_bytes().to_vec(),
        start: 0,
        current: 0,
        line: 1
    }
}