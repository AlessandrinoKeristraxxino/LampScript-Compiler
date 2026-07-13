// codegen.rs

use std::collections::HashMap;
use crate::parser::ast::{Expr, Stmt, Type};
use crate::lexer::token::TokenType;

#[derive(Clone)]
pub struct VarInfo {
    pub offset: i32,
    pub is_mutable: bool,
    pub var_type: Option<Type>,
}

pub struct Codegen {
    pub code: String,
    scopes: Vec<HashMap<String, VarInfo>>,
    stack_offset: i32,
    strings: Vec<(String, String)>,
    string_counter: usize,
    label_counter: usize,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            code: String::new(),
            scopes: vec![HashMap::new()],
            stack_offset: 0,
            strings: Vec::new(),
            string_counter: 0,
            label_counter: 0,
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn get_var(&self, name: &str) -> Option<&VarInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.get(name) {
                return Some(var);
            }
        }
        None
    }

    fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block(stmts) => {
                let prev_stack = self.stack_offset;
                self.push_scope();
                for s in stmts {
                    self.gen_stmt(s);
                }
                self.pop_scope();
                self.stack_offset = prev_stack;
            },
            Stmt::If { condition, then_branch, else_branch } => {
                let else_label = format!(".Lif_else_{}", self.label_counter);
                let end_label = format!(".Lif_end_{}", self.label_counter);
                self.label_counter += 1;

                self.gen_expr(condition);
                self.code.push_str("    test rax, rax\n");
                
                if else_branch.is_some() {
                    self.code.push_str(&format!("    je {}\n", else_label));
                } else {
                    self.code.push_str(&format!("    je {}\n", end_label));
                }

                self.gen_stmt(then_branch);

                if let Some(eb) = else_branch {
                    self.code.push_str(&format!("    jmp {}\n", end_label));
                    self.code.push_str(&format!("{}:\n", else_label));
                    self.gen_stmt(eb);
                }
                self.code.push_str(&format!("{}:\n", end_label));
            },
            Stmt::While { condition, body } => {
                let start_label = format!(".Lwhile_start_{}", self.label_counter);
                let end_label = format!(".Lwhile_end_{}", self.label_counter);
                self.label_counter += 1;

                self.code.push_str(&format!("{}:\n", start_label));
                self.gen_expr(condition);
                self.code.push_str("    test rax, rax\n");
                self.code.push_str(&format!("    je {}\n", end_label));

                self.gen_stmt(body);
                self.code.push_str(&format!("    jmp {}\n", start_label));
                self.code.push_str(&format!("{}:\n", end_label));
            },
            Stmt::Let { name, is_mutable, var_type, value } => {
                let inferred_type = match var_type {
                    Some(t) => Some(t.clone()),
                    None => match value {
                        Expr::StringLiteral(_) => Some(Type::String),
                        Expr::Number(n) => {
                            if n.fract() == 0.0 {
                                Some(Type::U64)
                            } else {
                                Some(Type::F64)
                            }
                        },
                        Expr::Identifier(id) => self.get_var(id).and_then(|v| v.var_type.clone()),
                        _ => Some(Type::U64),
                    }
                };

                self.gen_expr(value);

                self.stack_offset += 8;
                if let Some(scope) = self.scopes.last_mut() {
                    scope.insert(name.clone(), VarInfo {
                        offset: self.stack_offset,
                        is_mutable: *is_mutable,
                        var_type: inferred_type,
                    });
                }

                let instruction = format!("    mov [rbp - {}], rax\n", self.stack_offset);
                self.code.push_str(&instruction);
            },
            Stmt::Assign { name, value } => {
                if let Some(var_info) = self.get_var(name) {
                    if !var_info.is_mutable {
                        panic!("Compilation Error: Variable '{}' is not mutable!", name);
                    }
                    let offset = var_info.offset;
                    self.gen_expr(value);
                    let instruction = format!("    mov [rbp - {}], rax\n", offset);
                    self.code.push_str(&instruction);
                } else {
                    panic!("Compilation Error: Variable '{}' is not defined!", name);
                }
            },
            Stmt::Print(args) | Stmt::Println(args) => {
                if args.is_empty() { return; }

                let is_println = matches!(stmt, Stmt::Println(_));
                let mut format_str = String::new();
                let mut rest_args = &args[..];
                
                if let Expr::StringLiteral(s) = &args[0] {
                    format_str = s.clone();
                    rest_args = &args[1..];
                } else {
                    for _ in 0..args.len() {
                        format_str.push_str("{} ");
                    }
                }
                
                let mut n_args = 0;
                let mut final_fmt = String::new();
                let mut chars = format_str.chars().peekable();
                while let Some(c) = chars.next() {
                    if c == '{' && chars.peek() == Some(&'}') {
                        chars.next(); // consume '}'
                        
                        let arg_type = if n_args < rest_args.len() {
                            match &rest_args[n_args] {
                                Expr::StringLiteral(_) => Some(Type::String),
                                Expr::Number(n) => if n.fract() == 0.0 { Some(Type::U64) } else { Some(Type::F64) },
                                Expr::Identifier(name) => self.get_var(name).and_then(|v| v.var_type.clone()),
                                _ => Some(Type::U64)
                            }
                        } else {
                            Some(Type::U64)
                        };
                        
                        match arg_type {
                            Some(Type::String) => final_fmt.push_str("%s"),
                            Some(Type::F64) | Some(Type::F32) => final_fmt.push_str("%f"),
                            _ => final_fmt.push_str("%llu"),
                        }
                        n_args += 1;
                    } else {
                        final_fmt.push(c);
                    }
                }

                if is_println {
                    final_fmt.push_str("\\n");
                }
                
                let fmt_label = format!("fmt_{}", self.string_counter);
                self.string_counter += 1;
                
                let nasm_fmt = final_fmt.replace("\\n", "\", 10, \"");
                self.strings.push((fmt_label.clone(), format!("db \"{}\", 0", nasm_fmt)));

                let mut args_setup = Vec::new();
                for (i, arg) in rest_args.iter().enumerate() {
                    if i > 2 { break; } // Max 3 args supported right now for simplicity
                    let target_reg = match i {
                        0 => ("rdx", "xmm1"),
                        1 => ("r8", "xmm2"),
                        2 => ("r9", "xmm3"),
                        _ => unreachable!(),
                    };
                    args_setup.push((arg.clone(), target_reg));
                }

                for (arg, (gp_reg, xmm_reg)) in args_setup {
                    self.gen_expr(&arg);
                    let instruction = format!("    mov {}, rax\n    movq {}, rax\n", gp_reg, xmm_reg);
                    self.code.push_str(&instruction);
                }

                let instruction = format!("    lea rcx, [rel {}]\n    sub rsp, 32\n    call printf\n    add rsp, 32\n", fmt_label);
                self.code.push_str(&instruction);
            }
        }
    }

    fn gen_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(value) => {
                if value.fract() == 0.0 {
                    let instruction = format!("    mov rax, {}\n", *value as i64);
                    self.code.push_str(&instruction);
                } else {
                    let label = format!("float_{}", self.string_counter);
                    self.string_counter += 1;
                    self.strings.push((label.clone(), format!("dq {}", value)));
                    let instruction = format!("    movsd xmm0, [rel {}]\n    movq rax, xmm0\n", label);
                    self.code.push_str(&instruction);
                }
            },
            Expr::StringLiteral(s) => {
                let label = format!("str_{}", self.string_counter);
                self.string_counter += 1;
                self.strings.push((label.clone(), format!("db \"{}\", 0", s)));
                let instruction = format!("    lea rax, [rel {}]\n", label);
                self.code.push_str(&instruction);
            },
            Expr::Identifier(name) => {
                if let Some(var_info) = self.get_var(name) {
                    let instruction = format!("    mov rax, [rbp - {}]\n", var_info.offset);
                    self.code.push_str(&instruction);
                } else {
                    panic!("Compilation Error: Variable '{}' not defined!", name);
                }
            },
            Expr::Binary { left, op, right } => {
                self.gen_expr(left);
                self.code.push_str("    push rax\n");
                self.gen_expr(right);
                self.code.push_str("    pop rcx\n");
                // rcx = left, rax = right
                match op {
                    TokenType::Plus => self.code.push_str("    add rax, rcx\n"),
                    TokenType::Minus => {
                        self.code.push_str("    sub rcx, rax\n");
                        self.code.push_str("    mov rax, rcx\n");
                    },
                    TokenType::Asterisk => {
                        self.code.push_str("    imul rax, rcx\n");
                    },
                    TokenType::Slash => {
                        self.code.push_str("    xchg rax, rcx\n");
                        self.code.push_str("    cqo\n");
                        self.code.push_str("    idiv rcx\n"); // rax = left / right
                    },
                    TokenType::EqualEqual => {
                        self.code.push_str("    cmp rcx, rax\n    sete al\n    movzx rax, al\n");
                    },
                    TokenType::NotEqual => {
                        self.code.push_str("    cmp rcx, rax\n    setne al\n    movzx rax, al\n");
                    },
                    TokenType::LessThan => {
                        self.code.push_str("    cmp rcx, rax\n    setl al\n    movzx rax, al\n");
                    },
                    TokenType::GreaterThan => {
                        self.code.push_str("    cmp rcx, rax\n    setg al\n    movzx rax, al\n");
                    },
                    TokenType::LessThanOrEqual => {
                        self.code.push_str("    cmp rcx, rax\n    setle al\n    movzx rax, al\n");
                    },
                    TokenType::GreaterThanOrEqual => {
                        self.code.push_str("    cmp rcx, rax\n    setge al\n    movzx rax, al\n");
                    },
                    TokenType::And => {
                        self.code.push_str("    test rcx, rcx\n    setne cl\n    test rax, rax\n    setne al\n    and rax, rcx\n    movzx rax, al\n");
                    },
                    TokenType::Or => {
                        self.code.push_str("    test rcx, rcx\n    setne cl\n    test rax, rax\n    setne al\n    or rax, rcx\n    movzx rax, al\n");
                    },
                    TokenType::NotBoth => {
                        self.code.push_str("    cmp rcx, rax\n    setne r8b\n");
                        self.code.push_str("    test rcx, rcx\n    sete r9b\n");
                        self.code.push_str("    test rax, rax\n    sete r10b\n");
                        self.code.push_str("    and r8b, r9b\n    and r8b, r10b\n    movzx rax, r8b\n");
                    },
                    _ => panic!("Unsupported binary operator: {:?}", op)
                }
            },
            Expr::Unary { op: _, expr: _ } | Expr::Root { degree: _, expr: _ } => {
                panic!("Unary and Root compilation not fully implemented!");
            }
        }
    }

    pub fn compile(&mut self, statements: &[Stmt]) {
        let text_code = format!("
            global main\n\
            extern ExitProcess\n\
            extern printf\n\
            \n
            section .text\n\
            main:\n\
                push rbp\n\
                mov rbp, rsp\n\
                sub rsp, 256\n"
        );
        self.code.push_str(&text_code);

        for stmt in statements {
            self.gen_stmt(stmt);
        }

        let last_code = format!("
            mov rcx, 0\n\
            call ExitProcess\n"
        );
        self.code.push_str(&last_code);
        
        if !self.strings.is_empty() {
            self.code.push_str("\nsection .data\n");
            for (label, data) in &self.strings {
                self.code.push_str(&format!("    {} {}\n", label, data));
            }
        }
    }
}