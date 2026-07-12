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

use std::{fs::File, io::Write, path::PathBuf, process::Command};
use crate::lexer::tokenizer::Lexer;
use crate::parser::ast::Parser;
use crate::codegen::codegen::Codegen;

mod lexer;
mod parser;
mod codegen;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(1);
    });

    if let Err(err) = compile_file(&config.input_path) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn parse_args(args: &[String]) -> Result<Config, String> {
    let mut iter = args.iter().skip(1);
    let subcommand = iter.next().map(|value| value.as_str());

    match subcommand {
        None => Ok(Config::default()),
        Some("run") => {
            let path = iter.next().ok_or_else(|| "Usage: las run <file.las>".to_string())?;
            Ok(Config {
                input_path: PathBuf::from(path),
            })
        }
        Some("help") | Some("--help") | Some("-h") => Err("Usage: las [run <file.las>]".to_string()),
        Some(other) => Err(format!("Unknown command: {other}")),
    }
}

struct Config {
    input_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_path: PathBuf::from("program.las"),
        }
    }
}

fn compile_file(path: &PathBuf) -> Result<(), String> {
    let source_code = std::fs::read_to_string(path)
        .map_err(|_| format!("It is impossible to read the source file: {}", path.display()))?
        .chars()
        .collect();

    let mut lexer = Lexer::new(source_code);
    let tokens = lexer.lexing();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    let mut codegen = Codegen::new();
    codegen.compile(&program.statements);

    let assembly_code = codegen.code;
    std::fs::create_dir_all("target")
        .map_err(|err| format!("Failed to create target directory: {err}"))?;

    let asm_path = PathBuf::from("target/compiled.asm");
    let mut file = File::create(&asm_path)
        .map_err(|err| format!("Failed to create assembly file: {err}"))?;

    file.write_all(assembly_code.as_bytes())
        .map_err(|err| format!("Failed to write assembly file: {err}"))?;
    println!("Assembly code written to {}", asm_path.display());

    println!("NASM assembling...");
    let nasm_status = Command::new("nasm")
        .args(["-f", "win64", "target/compiled.asm", "-o", "target/compiled.obj"])
        .status();

    match nasm_status {
        Ok(status) if status.success() => {
            println!("✓ NASM assembling completed successfully.");

            println!("Linking...");
            let linker_candidates = [
                "gcc.exe",
                "gcc",
                "clang.exe",
                "clang",
                "clang++.exe",
                "clang++",
                "x86_64-w64-mingw32-gcc.exe",
                "x86_64-w64-mingw32-gcc",
            ];

            let linker_status = linker_candidates.iter().find_map(|linker| {
                let output = Command::new(linker)
                    .args(["target/compiled.obj", "-o", "target/compiled.exe"])
                    .output();

                match output {
                    Ok(output) if output.status.success() => Some(Ok(())),
                    Ok(output) => Some(Err(format!(
                        "❌ Error during linking with {linker}: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ))),
                    Err(err) => Some(Err(format!("❌ Failed to run linker {linker}: {err}"))),
                }
            });

            match linker_status {
                Some(Ok(())) => {
                    println!("Compilation completed successfully.");
                    println!("You can execute the compiled program using: .\\target\\compiled.exe");
                    Ok(())
                }
                Some(Err(err)) => Err(err),
                None => Err("❌ Error during linking. Install a linker such as GCC, Clang or MinGW and retry.".to_string()),
            }
        }
        _ => Err("Failed to assemble the program with NASM.".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_default_program_file_when_no_arguments_are_given() {
        let config = parse_args(&["las".to_string()]).unwrap();
        assert_eq!(config.input_path, PathBuf::from("program.las"));
    }

    #[test]
    fn parses_run_subcommand_with_input_file() {
        let config = parse_args(&["las".to_string(), "run".to_string(), "demo.las".to_string()]).unwrap();
        assert_eq!(config.input_path, PathBuf::from("demo.las"));
    }
}