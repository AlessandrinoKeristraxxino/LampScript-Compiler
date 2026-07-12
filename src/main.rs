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

use std::{fs::File, io::Write, process::Command, path::Path};
use crate::lexer::tokenizer::Lexer;
use crate::parser::ast::Parser;
use crate::codegen::codegen::Codegen;

mod lexer;
mod parser;
mod codegen;

fn main() {
    let source_code = std::fs::read_to_string("program.las")
        .expect("It's impossible to read the source file")
        .chars()
        .collect();

    let mut lexer = Lexer::new(source_code);
    let tokens = lexer.lexing();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    let mut codegen = Codegen::new();
    codegen.compile(&program.statements);

    let assembly_code = codegen.code;
    std::fs::write("./target/compiled.asm", &assembly_code)
        .expect("Failed to write the Assembly file");

    std::fs::create_dir_all("target").unwrap();

    let asm_path = "target/compiled.asm";
    let mut file = File::create(asm_path).unwrap();

    file.write_all(assembly_code.as_bytes()).unwrap();
    println!("Assembly code written to {}", asm_path);

    println!("NASM assembling...");
    let nasm_status = Command::new("nasm")
        .args(&["-f", "win64", "target/compiled.asm", "-o", "target/compiled.obj"])
        .status();

    match nasm_status {
        Ok(status) if status.success() => {
            println!("✓ NASM assembling completed successfully.");
            
            println!("GCC linking...");
            let linker_status = Command::new("gcc")
                .args(&[
                    "target/compiled.obj",
                    "-o",
                    "target/compiled.exe",
                ])
                .status();

            match linker_status {
                Ok(l_status) if l_status.success() => {
                    println!("🎉 COMPILAZIONE COMPLETATA CON SUCCESSO!");
                    println!("You can execute the compiled program using: .\\target\\compiled.exe");
                }
                _ => {
                    eprintln!("❌ Errore durante il linkaggio con GCC.");
                }
            }
        }
        _ => {
            eprintln!("Failed to assemble the program with NASM.");
        }
    }
}