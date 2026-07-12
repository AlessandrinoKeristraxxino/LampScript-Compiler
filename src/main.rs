// main.rs

/*
 * Linguaggio:
 * 
 * print?(); -> funzione (inizialmente, solo per adesso pk non so come fare) poi macro
 * println?(); -> funzione, poi macro anche questa
 * 
 * let -> dichiarazione variabili immutabili
 * let x = 10; per adesso solo unsigned int
 */

use std::fs;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::codegen::Codegen;

mod lexer;
mod parser;
mod codegen;

fn main() {
    let source_code = fs::read_to_string("program.las")
        .expect("It's impossible to read the source file");

    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.lexing();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    let mut codegen = Codegen::new();
    codegen.compile(&program.statements);

    let assembly_code = codegen.code;
    fs::write("./target/compiled.asm", &assembly_code)
        .expect("Failed to write the Assembly file");
}