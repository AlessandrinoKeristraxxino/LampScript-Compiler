// codegen.rs

use std::collections::HashMap;
use crate::parser::ast::{Expr, Stmt, Type};
use crate::lexer::token::TokenType;

#[derive(Clone, PartialEq, Debug)]
pub enum VarState {
    Valid,
    Moved,
}

#[derive(Clone)]
pub struct VarInfo {
    pub offset: i32,
    pub is_global: bool,
    pub global_label: String,
    pub is_mutable: bool,
    pub var_type: Option<Type>,
    pub state: VarState,
    pub is_borrowed: bool,
    pub is_allocated: bool,
}

pub struct Codegen {
    pub code: String,
    scopes: Vec<HashMap<String, VarInfo>>,
    stack_offset: i32,
    strings: Vec<(String, String)>,
    bss: Vec<(String, String)>,
    string_counter: usize,
    label_counter: usize,
    current_return_type: Option<Type>,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            code: String::new(),
            scopes: vec![HashMap::new()], // Global scope
            stack_offset: 0,
            strings: Vec::new(),
            bss: Vec::new(),
            string_counter: 0,
            label_counter: 0,
            current_return_type: None,
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

    fn get_var_mut(&mut self, name: &str) -> Option<&mut VarInfo> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                return scope.get_mut(name);
            }
        }
        None
    }

    fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Function { name, params, return_type, body } => {
                // Functions are generated separately
            },
            Stmt::Block(stmts) => {
                let prev_stack = self.stack_offset;
                self.push_scope();
                for s in stmts {
                    self.gen_stmt(s);
                }
                if let Some(scope) = self.scopes.last() {
                    for (_name, var_info) in scope.iter() {
                        if var_info.state == VarState::Valid && var_info.is_allocated {
                            let free_code = format!(
                                "    mov rcx, [rel global_heap_handle]\n    mov rdx, 0\n    mov r8, [rbp - {}]\n    sub rsp, 32\n    call HeapFree\n    add rsp, 32\n",
                                var_info.offset
                            );
                            self.code.push_str(&free_code);
                        }
                    }
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
            Stmt::Return(expr) => {
                if let Some(t) = &self.current_return_type {
                    if *t == Type::Void && expr.is_some() {
                        panic!("Compilation Error: Cannot return a value from a void function");
                    }
                }
                
                if let Some(e) = expr {
                    self.gen_expr(e);
                }
                self.code.push_str("    mov rsp, rbp\n    pop rbp\n    ret\n");
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
                        Expr::Call { .. } => Some(Type::U64), // Simplified
                        _ => Some(Type::U64),
                    }
                };

                let mut is_alloc = matches!(value, Expr::Alloc(_));
                if let Expr::Identifier(id_name) = &value {
                    if let Some(v) = self.get_var(id_name) {
                        is_alloc = v.is_allocated;
                    }
                }

                self.gen_expr(value);

                let is_global = self.scopes.len() == 1;

                if is_global {
                    let global_label = format!("global_{}", name);
                    self.bss.push((global_label.clone(), "resq 1".to_string()));
                    
                    if let Some(scope) = self.scopes.first_mut() {
                        scope.insert(name.clone(), VarInfo {
                            offset: 0,
                            is_global: true,
                            global_label: global_label.clone(),
                            is_mutable: *is_mutable,
                            var_type: inferred_type.clone(),
                            state: VarState::Valid,
                            is_borrowed: false,
                            is_allocated: is_alloc,
                        });
                    }
                    self.code.push_str(&format!("    mov [rel {}], rax\n", global_label));
                } else {
                    self.stack_offset += 8;
                    if let Some(scope) = self.scopes.last_mut() {
                        scope.insert(name.clone(), VarInfo {
                            offset: self.stack_offset,
                            is_global: false,
                            global_label: String::new(),
                            is_mutable: *is_mutable,
                            var_type: inferred_type.clone(),
                            state: VarState::Valid,
                            is_borrowed: false,
                            is_allocated: is_alloc,
                        });
                    }
                    self.code.push_str(&format!("    mov [rbp - {}], rax\n", self.stack_offset));
                }
            },
            Stmt::Assign { name, value } => {
                let mut alloc_status = matches!(value, Expr::Alloc(_));
                if let Expr::Identifier(id_name) = &value {
                    if let Some(v) = self.get_var(id_name) {
                        alloc_status = v.is_allocated;
                    }
                }

                let (is_global, global_label, offset) = {
                    let var_info = self.get_var_mut(name).unwrap_or_else(|| panic!("Compilation Error: Variable '{}' is not defined!", name));
                    if !var_info.is_mutable {
                        panic!("Security Error: Cannot mutate immutable variable '{}'!", name);
                    }
                    var_info.state = VarState::Valid;
                    var_info.is_allocated = alloc_status;
                    (var_info.is_global, var_info.global_label.clone(), var_info.offset)
                };
                
                self.gen_expr(value);
                if is_global {
                    self.code.push_str(&format!("    mov [rel {}], rax\n", global_label));
                } else {
                    self.code.push_str(&format!("    mov [rbp - {}], rax\n", offset));
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
                
                let mut final_args = Vec::new();
                let mut n_args = 0;
                let mut final_fmt = String::new();
                let mut chars = format_str.chars().peekable();
                while let Some(c) = chars.next() {
                    if c == '{' {
                        let mut var_name = String::new();
                        while let Some(&next_c) = chars.peek() {
                            if next_c == '}' {
                                chars.next();
                                break;
                            }
                            var_name.push(chars.next().unwrap());
                        }

                        let expr = if var_name.is_empty() {
                            let e = rest_args.get(n_args).cloned().unwrap_or(Expr::Number(0.0));
                            n_args += 1;
                            e
                        } else {
                            Expr::Identifier(var_name.clone())
                        };

                        let arg_type = match &expr {
                            Expr::StringLiteral(_) => Some(Type::String),
                            Expr::Number(n) => if n.fract() == 0.0 { Some(Type::U64) } else { Some(Type::F64) },
                            Expr::Identifier(name) => self.get_var(name).and_then(|v| v.var_type.clone()),
                            Expr::Borrow(name) => self.get_var(name).and_then(|v| v.var_type.clone()),
                            Expr::Call { .. } => Some(Type::U64),
                            _ => Some(Type::U64)
                        };
                        
                        match arg_type {
                            Some(Type::String) => final_fmt.push_str("%s"),
                            Some(Type::F64) | Some(Type::F32) => final_fmt.push_str("%f"),
                            _ => final_fmt.push_str("%llu"),
                        }
                        
                        final_args.push(expr);
                    } else {
                        final_fmt.push(c);
                    }
                }
                
                let fmt_label = format!("fmt_{}", self.string_counter);
                self.string_counter += 1;

                if is_println {
                    self.strings.push((fmt_label.clone(), format!("db \"{}\", 10, 0", final_fmt)));
                } else {
                    self.strings.push((fmt_label.clone(), format!("db \"{}\", 0", final_fmt)));
                }

                let mut args_setup = Vec::new();
                for (i, arg) in final_args.iter().enumerate() {
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
            },
            Stmt::Expr(expr) => {
                self.gen_expr(expr);
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
                let (offset, global_label, is_global, is_borrowed) = {
                    let var_info = self.get_var_mut(name).unwrap_or_else(|| panic!("Compilation Error: Variable '{}' not defined!", name));
                    if var_info.state == VarState::Moved {
                        panic!("Security Error: Use of moved variable '{}'", name);
                    }
                    
                    let mut is_borrowed_val = false;
                    let is_copy = var_info.var_type.as_ref().map(|t| t.is_copy()).unwrap_or(true);
                    
                    if !is_copy || var_info.is_allocated {
                        var_info.state = VarState::Moved;
                    }
                    
                    if var_info.is_borrowed {
                        is_borrowed_val = true;
                    }
                    
                    (var_info.offset, var_info.global_label.clone(), var_info.is_global, is_borrowed_val)
                };

                if is_global {
                    self.code.push_str(&format!("    mov rax, [rel {}]\n", global_label));
                } else {
                    self.code.push_str(&format!("    mov rax, [rbp - {}]\n", offset));
                }
                
                if is_borrowed {
                    self.code.push_str("    mov rax, [rax]\n");
                }
            },
            Expr::Alloc(inner) => {
                self.gen_expr(inner);
                self.code.push_str("    push rax\n");
                self.code.push_str("    mov rcx, [rel global_heap_handle]\n");
                self.code.push_str("    mov rdx, 8\n"); // HEAP_ZERO_MEMORY = 8
                self.code.push_str("    mov r8, 8\n"); // 8 bytes allocation
                self.code.push_str("    sub rsp, 32\n    call HeapAlloc\n    add rsp, 32\n");
                self.code.push_str("    pop rcx\n");
                self.code.push_str("    mov [rax], rcx\n");
            },
            Expr::Borrow(name) => {
                if let Some(var_info) = self.get_var(name) {
                    if var_info.is_global {
                        self.code.push_str(&format!("    lea rax, [rel {}]\n", var_info.global_label));
                    } else {
                        self.code.push_str(&format!("    lea rax, [rbp - {}]\n", var_info.offset));
                    }
                } else {
                    panic!("Compilation Error: Variable '{}' not defined for borrowing!", name);
                }
            },
            Expr::Call { name, args } => {
                for (i, arg) in args.iter().enumerate() {
                    self.gen_expr(arg);
                    match i {
                        0 => self.code.push_str("    mov rcx, rax\n"),
                        1 => self.code.push_str("    mov rdx, rax\n"),
                        2 => self.code.push_str("    mov r8, rax\n"),
                        3 => self.code.push_str("    mov r9, rax\n"),
                        _ => self.code.push_str("    push rax\n"),
                    }
                }
                
                self.code.push_str("    sub rsp, 32\n");
                self.code.push_str(&format!("    call {}\n", name));
                self.code.push_str("    add rsp, 32\n");
                
                if args.len() > 4 {
                    self.code.push_str(&format!("    add rsp, {}\n", (args.len() - 4) * 8));
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
                        self.code.push_str("    test rcx, rcx\n    setne dl\n    test rax, rax\n    setne al\n    and al, dl\n    movzx rax, al\n");
                    },
                    TokenType::Or => {
                        self.code.push_str("    test rcx, rcx\n    setne dl\n    test rax, rax\n    setne al\n    or al, dl\n    movzx rax, al\n");
                    },
                    TokenType::NotBoth => {
                        self.code.push_str("    test rcx, rcx\n    setne dl\n    test rax, rax\n    setne al\n    and al, dl\n    xor al, 1\n    movzx rax, al\n");
                    },
                    _ => panic!("Compilation Error: Unknown binary operator: {:?}", op)
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
            extern GetProcessHeap\n\
            extern HeapAlloc\n\
            extern HeapFree\n\
            \n
            section .text\n"
        );
        self.code.push_str(&text_code);
        self.bss.push(("global_heap_handle".to_string(), "resq 1".to_string()));

        // Extract functions
        let mut fns = Vec::new();
        let mut top_level = Vec::new();
        
        for stmt in statements {
            if matches!(stmt, Stmt::Function { .. }) {
                fns.push(stmt);
            } else {
                top_level.push(stmt);
            }
        }

        // Generate main
        self.code.push_str("main:\n    push rbp\n    mov rbp, rsp\n    sub rsp, 256\n");
        self.code.push_str("    sub rsp, 32\n    call GetProcessHeap\n    add rsp, 32\n    mov [rel global_heap_handle], rax\n");
        self.stack_offset = 0;
        
        for stmt in top_level {
            self.gen_stmt(stmt);
        }

        let last_code = format!("
            mov rcx, 0\n\
            call ExitProcess\n"
        );
        self.code.push_str(&last_code);

        // Generate functions
        for f in fns {
            if let Stmt::Function { name, params, return_type, body } = f {
                self.code.push_str(&format!("{}:\n", name));
                self.code.push_str("    push rbp\n    mov rbp, rsp\n    sub rsp, 256\n");
                
                self.push_scope();
                self.stack_offset = 0; // reset local stack
                self.current_return_type = Some(return_type.clone());

                for (i, (p_name, is_mut, is_borrowed, p_type)) in params.iter().enumerate() {
                    self.stack_offset += 8;
                    
                    match i {
                        0 => self.code.push_str(&format!("    mov [rbp - {}], rcx\n", self.stack_offset)),
                        1 => self.code.push_str(&format!("    mov [rbp - {}], rdx\n", self.stack_offset)),
                        2 => self.code.push_str(&format!("    mov [rbp - {}], r8\n", self.stack_offset)),
                        3 => self.code.push_str(&format!("    mov [rbp - {}], r9\n", self.stack_offset)),
                        _ => {
                            // read from caller's stack
                            let caller_offset = 16 + (i - 4) * 8; // skip ret IP and saved RBP
                            self.code.push_str(&format!("    mov rax, [rbp + {}]\n", caller_offset));
                            self.code.push_str(&format!("    mov [rbp - {}], rax\n", self.stack_offset));
                        }
                    }

                    if let Some(scope) = self.scopes.last_mut() {
                        scope.insert(p_name.clone(), VarInfo {
                            offset: self.stack_offset,
                            is_global: false,
                            global_label: String::new(),
                            is_mutable: *is_mut,
                            var_type: Some(p_type.clone()),
                            state: VarState::Valid,
                            is_borrowed: *is_borrowed,
                            is_allocated: false, // function params don't allocate new heap
                        });
                    }
                }

                self.gen_stmt(body);

                if let Some(scope) = self.scopes.last() {
                    for (name, var_info) in scope.iter() {
                        if var_info.state == VarState::Valid && var_info.is_allocated {
                            let free_code = format!(
                                "    mov rcx, [rel global_heap_handle]\n    mov rdx, 0\n    mov r8, [rbp - {}]\n    sub rsp, 32\n    call HeapFree\n    add rsp, 32\n",
                                var_info.offset
                            );
                            self.code.push_str(&free_code);
                        }
                    }
                }
                
                self.pop_scope();
                self.current_return_type = None;

                // Emitted epilogue if no return was hit
                self.code.push_str("    mov rsp, rbp\n    pop rbp\n    ret\n\n");
            }
        }
        
        if !self.strings.is_empty() || !self.bss.is_empty() {
            self.code.push_str("\nsection .data\n");
            for (label, data) in &self.strings {
                self.code.push_str(&format!("    {} {}\n", label, data));
            }
            if !self.bss.is_empty() {
                self.code.push_str("\nsection .bss\n");
                for (label, data) in &self.bss {
                    self.code.push_str(&format!("    {} {}\n", label, data));
                }
            }
        }
    }
}