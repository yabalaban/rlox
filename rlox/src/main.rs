use std::env;
use std::fs;
use std::process;

use vm::Interpreter;
use vm::InterpreterDebug;
use vm::InterpretResult;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vm = vm::make();
    match args.len() {
        1 => { repl(&mut vm) },
        2 => { run_file(&mut vm, &args[1]) },
        _ => {
            println!("Usage: clox [path]");
            process::exit(64);
        }
    }
}

fn repl(vm: &mut (impl Interpreter + InterpreterDebug)) {
    use std::io::{self, BufRead, Write};
    use colored::*;

    let mut line = 1;
    println!("Welcome to Lox version 0.1.0 (written in Rust)");
    loop {
        print!("{}", format!("{:3}> ", line).truecolor(128, 128, 128));
        line += 1;
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();
        if let Some(line) = lines.next() {
            let line = line.expect("Error: unable to read user input");
            if line == "@stack" {
                vm.print_stack();
                break;
            }
            if line == "@code" {
                vm.print_code();
                break;
            }
            let res = &vm.interpret(&line);
            match res {
                InterpretResult::CompilationError => {
                    println!("{}", "Invalid syntax".red());
                },
                InterpretResult::RuntimeError => {
                    println!("{}", "Runtime error".red());
                },
                _ => {
                    if line.len() > 0 {
                        println!("{}", line);
                    }
                },
            }
        }
    }
}

fn run_file(vm: &mut (impl Interpreter + InterpreterDebug), path: &String) {
    let contents = fs::read_to_string(path)
        .expect("Error: unable to read file");
    let res = &vm.interpret(&contents);
    match res {
        InterpretResult::CompilationError => {
            process::exit(65);
        },
        InterpretResult::RuntimeError => {
            process::exit(70);
        },
        _ => {},
    }
}