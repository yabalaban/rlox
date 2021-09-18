use common::chunk;
use scanner::token;

// struct Parser {
//     previous: token::Token,
//     current: token::Token,
//     had_error: bool,
// }

pub fn compile(_chunk: &mut chunk::Chunk, source: &String) -> bool {
    let mut scanner = scanner::make(source);
    let mut line = 0;

    while let Some(token) = scanner.next() {
        let desc = format!("{:?} {:?}", token.pos, token.ty);
        if token.line != line {
            line = token.line;
            println!("{:4} {}", line, desc);
        } else {
            println!("   | {}", desc);
        }

        if matches!(token.ty, token::Type::Eof) {
            return true;
        }
    }
    false
}

// fn advance(parser: &mut Parser) {
    // parser.previous = parser.current;
// }