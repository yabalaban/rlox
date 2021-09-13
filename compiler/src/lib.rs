use scanner::token;
use common::opcode::OpCode;

pub fn compile(source: &String) -> Option<Vec<OpCode>> {
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
            return None;
        }
    }

    panic!("EOF is missing in input");
}