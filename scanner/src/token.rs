use std::ops::Range;

#[derive(Debug)]
pub enum Type {
    /* Single character tokens */
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,
    /* One-two character tokens */
    Bang, BangEqual, 
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    /* Literals */
    Identifier, String, Number,
    /* Keywords */
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    Error, Eof
}

#[derive(Debug)]
pub struct Token {
    pub ty: Type,
    pub pos: Range<usize>, 
    pub line: usize,
}