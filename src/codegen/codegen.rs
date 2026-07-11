// codegen.rs

use std::collections::HashMap;
use crate::ast::{Expr, Stmt, Program};

pub struct Codegen {
    code: String,
    variables: HashMap<String, i32>,
    stack_offset: i32,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            code: String::new(),
            variables: HashMap::new(),
            stack_offset: 0,
        }
    }

    fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, value } => {
                self.gen_expr(value);

                self.stack_offset += 8;
                self.variables.insert(name.clone(), self.stack_offset);

                let instruction = format!("    mov [rbp - {}], rax\n", self.stack_offset);
                self.code.push_str(&instruction);
            },
            _ => {}
        }
    }

    fn gen_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(value) => {
                let instruction = format!("    mov rax, {}\n", value);
                self.code.push_str(&instruction);
            },
            Expr::Identifier(name) => {
                if let Some(offset) = self.variables.get(name) {
                    let instruction = format!("    mov rax, [rbp - {}]\n", offset);
                    self.code.push_str(&instruction);
                } else {
                    panic!("Errore di compilazione: Variabile '{}' non definita!", name);
                }
            }
        }
    }
}