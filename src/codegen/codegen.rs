// codegen.rs

use std::collections::HashMap;
use crate::parser::ast::{Expr, Stmt};

pub struct Codegen {
    pub code: String,
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
            Stmt::Print(expr) => {
                self.gen_expr(expr);

                let instruction = format!("
                    mov rdx, rax\n\
                    lea rcx, [rel format_num]\n\
                    sub rsp, 32\n\
                    call printf\n\
                    add rsp, 32"
                );
                self.code.push_str(&instruction);
            },
            Stmt::Println(expr) => {
                self.gen_expr(expr);

                let instruction = format!("
                    mov rdx, rax\n\
                    lea rcx, [rel format_nl]\n\
                    sub rsp, 32\n\
                    call printf\n\
                    add rsp, 32"
                );
                self.code.push_str(&instruction);
            }
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
            },
            _ => {}
        }
    }

    pub fn compile(&mut self, statements: &[Stmt]) {
        let first_code = format!("
            global main\n\
            extern ExitProcess\n\
            extern printf\n\
            \n
            section .data\n\
                format_num db \"%llu\", 0\n\
                format_nl  db \"%llu\", 10, 0\n\
            \n
            section .text\n\
            main:\n\
                push rbp\n\
                mov rbp, rsp\n\
                sub rsp, 256\n"
        );
        self.code.push_str(&first_code);

        for stmt in statements {
            self.gen_stmt(stmt);
        }

        let last_code = format!("
            mov rcx, 0\n\
            call ExitProcess"
        );
        self.code.push_str(&last_code);
    }
}