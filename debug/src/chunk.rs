use common::chunk::Chunk;
use common::opcode::ByteCode;
use crate::value;

pub fn disassemble(chunk: &Chunk) {
    println!("== Chunk ==");

    let mut offset = 0;
    loop {
        offset = disassemble_opcode(chunk, offset);
        if offset >= chunk.code.len() {
            break;
        }
    }
}

fn disassemble_opcode(chunk: &Chunk, offset: usize) -> usize {
    let opcode = chunk.code[offset];
    let line = if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        "   |".to_string()
    } else {
        format!("{:4}", chunk.lines[offset])
    };
    let (desc, width) = match chunk.code[offset] {
        ByteCode::RETURN => simple_opcode("OP_RETURN".to_string()),
        ByteCode::NEGATE => simple_opcode("OP_NEGATE".to_string()),
        ByteCode::ADD => simple_opcode("OP_ADD".to_string()),
        ByteCode::SUBTRACT => simple_opcode("OP_SUBTRACT".to_string()),
        ByteCode::MULTIPLY => simple_opcode("OP_MULTIPLY".to_string()),
        ByteCode::DIVIDE => simple_opcode("OP_DIVIDE".to_string()),
        ByteCode::CONSTANT => constant_opcode(chunk, offset),
        _ => panic!("OpCode \"{}\" is not defined", opcode),
    };
    println!("{:04} {} {}", offset, line, desc);
    offset + width
}

fn simple_opcode(name: String) -> (String, usize) {
    (name, 1)
}

fn constant_opcode(chunk: &Chunk, offset: usize) -> (String, usize) {
    let width = common::opcode::offset(chunk.code[offset]) as usize;
    let value = chunk.code[offset + 1];
    let constant = value::description(chunk.constants[value as usize]);
    return (format!("{:16} {:4} \'{}\'", "OP_CONSTANT", value, constant), width)
}