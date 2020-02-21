use ast::{Expr, Meta, Operator, Parameter, Program, Stmt};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use util::get_pos;

const OFFSET: usize = 6010000;
const CURRENT_FUNCTION_CALL: &str = "CURRENTFUNCTION";

type SymbolTables = Vec<HashMap<String, SymbolEntry>>;

#[derive(Debug, Clone)]
pub struct CompilationError {
    msg: String,
}

impl CompilationError {
    fn new(msg: String) -> CompilationError {
        CompilationError { msg: msg }
    }
}

impl fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for CompilationError {
    fn description(&self) -> &str {
        self.msg.as_str()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SymbolEntry {
    mem_loc: usize,
    param_locs: Option<Vec<usize>>,
    parameters: Option<Vec<Parameter>>,
}

impl SymbolEntry {
    pub fn new(
        count: &mut usize,
        param_locs: Option<Vec<usize>>,
        parameters: Option<Vec<Parameter>>,
    ) -> SymbolEntry {
        *count += 4;
        SymbolEntry {
            mem_loc: *count,
            param_locs: param_locs,
            parameters: parameters,
        }
    }
}

pub struct Compiler {
    pub stmts: Vec<String>,
    pub symbol_tables: SymbolTables,
    pub count: usize,
    pub source_code: String,
}

impl Compiler {
    pub fn new(source_code: String) -> Compiler {
        Compiler {
            stmts: vec![],
            symbol_tables: vec![HashMap::new()],
            count: 0,
            source_code: source_code,
        }
    }

    pub fn compile(&mut self, program: &Program) -> Result<Vec<String>, CompilationError> {
        self.stmts.push(format!("b functionmain"));
        for statement in program {
            self.compile_stmt(statement)?;
        }

        self.stmts.push(format!("swi 0x11"));

        let mut symbol_table_comments = vec![];
        symbol_table_comments.push("; Symbol table:".to_string());
        for symbol_table in self.symbol_tables.iter() {
            for (k, symbol) in symbol_table.iter() {
                symbol_table_comments.push(format!("; {}={:?}", k, symbol));
            }
        }

        Ok([symbol_table_comments.as_slice(), self.stmts.as_slice()].concat())
    }

    fn compile_stmt(&mut self, stmt: &Meta<Stmt>) -> Result<(), CompilationError> {
        self.stmts.push(format!(
            "\n; line {}",
            get_pos(&self.source_code, stmt.byte_offset).0
        ));

        match stmt.inside {
            Stmt::Block(ref statements) => {
                self.symbol_tables.push(HashMap::new());
                for stmt in statements {
                    self.compile_stmt(&stmt)?;
                }
                self.symbol_tables.pop();
            }
            Stmt::Declaration(ref parameters) => {
                for parameter in parameters {
                    if let Some(table_for_scope) = self.symbol_tables.last_mut() {
                        table_for_scope.insert(
                            parameter.identifier.clone(),
                            SymbolEntry::new(&mut self.count, None, Some(vec![parameter.clone()])),
                        );
                    }
                }
            }
            Stmt::Function(ref name, ref parameters, ref statement) => {
                let mut param_locs = vec![];
                for parameter in parameters {
                    if let Some(table_for_scope) = self.symbol_tables.last_mut() {
                        table_for_scope.insert(
                            parameter.identifier.clone(),
                            SymbolEntry::new(&mut self.count, None, None),
                        );
                        param_locs.push(self.count);
                    }
                }
                self.count += 4;
                param_locs.push(self.count);

                if let Some(table_for_scope) = self.symbol_tables.last_mut() {
                    table_for_scope.insert(
                        name.clone(),
                        SymbolEntry::new(
                            &mut self.count,
                            Some(param_locs),
                            Some(parameters.clone()),
                        ),
                    );
                }

                self.count += 4;
                let return_loc = self.count;
                let label = format!("function{}", name);
                self.stmts.push(format!("{}:", label));
                self.stmts.push(format!("str lr, ={}", OFFSET + return_loc));
                self.compile_stmt(&*statement)?;
                self.stmts.push(format!("ldr r0, ={}", OFFSET + return_loc));
                self.stmts.push(format!("mov pc, r0"));
            }
            Stmt::Write(ref expr) => {
                let expr_val = self.compile_expr(&expr)?;
                self.stmts.push(format!("mov r0, #1")); // configure for stdout
                self.stmts.push(format!("ldr r1, ={}", OFFSET + expr_val));
                self.stmts.push(format!("swi 0x6b")); // print val in r1
            }
            Stmt::Read(ref name) => {
                if let Some(var) = get_var(name.to_string(), &self.symbol_tables) {
                    self.stmts.push(format!("mov r0, #0")); // configure for stdin
                    self.stmts.push(format!("swi 0x6c")); // read val to r0
                    self.stmts
                        .push(format!("str r0, ={}", OFFSET + var.mem_loc));
                } else {
                    let pos = get_pos(&self.source_code, stmt.byte_offset);
                    return Err(CompilationError::new(format!(
                        "Variable not found at line {}",
                        pos.0,
                    )));
                }
            }
            Stmt::Expr(ref expr) => {
                self.compile_expr(&expr)?;
            }
            Stmt::If(ref expr, ref statement) => {
                let end = format!("ifend{}", stmt.byte_offset);
                let expr_val = self.compile_expr(&*expr)?;
                self.stmts.push(format!("ldr r1, ={}", OFFSET + expr_val));
                self.stmts.push(format!("cmp r1, #0"));
                self.stmts.push(format!("beq {}", end));
                self.compile_stmt(&*statement)?;
                self.stmts.push(format!("{}:", end));
            }
            Stmt::While(ref expr, ref statement) => {
                let start = format!("whilestart{}", stmt.byte_offset);
                let end = format!("whileend{}", stmt.byte_offset);
                self.stmts.push(format!("{}:", start));
                let expr_val = self.compile_expr(&*expr)?;
                self.stmts.push(format!("ldr r1, ={}", OFFSET + expr_val));
                self.stmts.push(format!("cmp r1, #0"));
                self.stmts.push(format!("beq {}", end));
                self.compile_stmt(&*statement)?;
                self.stmts.push(format!("b {}", start));
                self.stmts.push(format!("{}:", end));
            }
            Stmt::For(ref e1, ref e2, ref e3, ref statement) => {
                let start = format!("forstart{}", stmt.byte_offset);
                let end = format!("forend{}", stmt.byte_offset);
                self.compile_expr(&e1)?;
                self.stmts.push(format!("{}:", start));
                let e2_val = self.compile_expr(&e2)?;
                self.stmts.push(format!("ldr r1, ={}", OFFSET + e2_val));
                self.stmts.push(format!("cmp r1, #0"));
                self.stmts.push(format!("beq {}", end));
                self.compile_stmt(&*statement)?;
                self.compile_expr(&e3)?;
                self.stmts.push(format!("b {}", start));
                self.stmts.push(format!("{}:", end));
            }
            Stmt::Return(ref expr) => {
                let expr_val = self.compile_expr(&expr)?;
                if let Some(ref function_def) = get_function_def(&self.symbol_tables) {
                    if let Some(ref param_locs) = function_def.param_locs {
                        let return_loc = param_locs.get(0).unwrap();
                        let return_val_loc = param_locs.get(1).unwrap();
                        self.stmts.push(format!("ldr r0, ={}", OFFSET + expr_val));
                        self.stmts
                            .push(format!("str r0, ={}", OFFSET + return_val_loc));
                        self.stmts.push(format!("ldr r0, ={}", OFFSET + return_loc));
                        self.stmts.push(format!("mov pc, r0"));
                    }
                }
            }
        }
        Ok(())
    }

    fn compile_expr(&mut self, expr: &Meta<Expr>) -> Result<usize, CompilationError> {
        self.count += 4;
        let curr_mem_loc = self.count;
        let mut return_mem = curr_mem_loc;
        match expr.inside {
            Expr::Binary(ref op, ref e1, ref e2) => {
                match op {
                    Operator::LogicalAnd => {
                        let label1 = format!(".And1{}", curr_mem_loc);
                        let label2 = format!(".And2{}", curr_mem_loc);
                        let e1_loc = self.compile_expr(&*e1)?;
                        self.stmts.push(format!("ldr r2, ={}", OFFSET + e1_loc));
                        self.stmts.push(format!("cmp r2, #0"));
                        self.stmts.push(format!("beq {}", label1));

                        let e2_loc = self.compile_expr(&*e2)?;
                        self.stmts.push(format!("ldr r3, ={}", OFFSET + e2_loc));

                        self.stmts.push(format!("cmp r3, #0"));
                        self.stmts.push(format!("beq {}", label1));
                        self.stmts.push(format!("mov r3, #1"));
                        self.stmts.push(format!("b {}", label2));

                        self.stmts.push(format!("{}:", label1));
                        self.stmts.push(format!("mov r3, #0"));
                        self.stmts.push(format!("{}:", label2));
                        self.stmts
                            .push(format!("str r3, ={}", OFFSET + curr_mem_loc));
                    }
                    Operator::LogicalOr => {
                        let label1 = format!(".Or1{}", curr_mem_loc);
                        let label2 = format!(".Or2{}", curr_mem_loc);
                        let label3 = format!(".Or3{}", curr_mem_loc);
                        let e1_loc = self.compile_expr(&*e1)?;
                        self.stmts.push(format!("ldr r2, ={}", OFFSET + e1_loc));
                        self.stmts.push(format!("cmp r2, #0"));
                        self.stmts.push(format!("bne {}", label1));

                        let e2_loc = self.compile_expr(&*e2)?;
                        self.stmts.push(format!("ldr r3, ={}", OFFSET + e2_loc));

                        self.stmts.push(format!("cmp r3, #0"));
                        self.stmts.push(format!("beq {}", label2));

                        self.stmts.push(format!("{}:", label1));
                        self.stmts.push(format!("mov r3, #1"));
                        self.stmts.push(format!("b {}", label3));

                        self.stmts.push(format!("{}:", label2));
                        self.stmts.push(format!("mov r3, #0"));

                        self.stmts.push(format!("{}:", label3));
                        self.stmts
                            .push(format!("str r3, ={}", OFFSET + curr_mem_loc));
                    }
                    Operator::Greater
                    | Operator::GreaterEqual
                    | Operator::Less
                    | Operator::LessEqual
                    | Operator::Equal
                    | Operator::NotEqual => {
                        let e1_loc = self.compile_expr(&*e1)?;
                        let e2_loc = self.compile_expr(&*e2)?;
                        self.stmts.push(format!("mov r1, #0"));
                        self.stmts.push(format!("ldr r2, ={}", OFFSET + e1_loc));
                        self.stmts.push(format!("ldr r3, ={}", OFFSET + e2_loc));
                        self.stmts.push(format!("cmp r2, r3"));
                        let instruction = match op {
                            Operator::Greater => "movgt",
                            Operator::GreaterEqual => "movge",
                            Operator::Less => "movlt",
                            Operator::LessEqual => "movle",
                            Operator::Equal => "moveq",
                            Operator::NotEqual => "movne",
                            _ => "",
                        };
                        self.stmts.push(format!("{} r1, #1", instruction));
                        self.stmts
                            .push(format!("str r1, ={}", OFFSET + curr_mem_loc));
                    }
                    Operator::Divide | Operator::Mod => {
                        let label_start = format!(".start{}", curr_mem_loc);
                        let label_end = format!(".end{}", curr_mem_loc);

                        let e1_loc = self.compile_expr(&*e1)?;
                        let e2_loc = self.compile_expr(&*e2)?;
                        self.stmts.push(format!("ldr r1, ={}", OFFSET + e1_loc));
                        self.stmts.push(format!("ldr r2, ={}", OFFSET + e2_loc));
                        self.stmts.push(format!("mov r3, #0"));

                        self.stmts.push(format!("cmp r1, r2"));
                        self.stmts.push(format!("blt {}", label_end));

                        self.stmts.push(format!("{}:", label_start));
                        self.stmts.push(format!("sub r1, r1, r2"));
                        self.stmts.push(format!("add r3, r3, #1"));

                        self.stmts.push(format!("cmp r1, r2"));
                        self.stmts.push(format!("bge {}", label_start));
                        self.stmts.push(format!("{}:", label_end));
                        // r3 = quotient
                        // r1 = remainder

                        if let Operator::Divide = op {
                            self.stmts
                                .push(format!("str r3, ={}", OFFSET + curr_mem_loc));
                        } else {
                            self.stmts
                                .push(format!("str r1, ={}", OFFSET + curr_mem_loc));
                        }
                    }
                    _ => {
                        let e1_val = self.compile_expr(&*e1)?;
                        let e2_val = self.compile_expr(&*e2)?;
                        let instruction_name = match op {
                            Operator::Add => "add",
                            Operator::Subtract => "sub",
                            Operator::Multiply => "mul",
                            Operator::And => "and",
                            Operator::Or => "orr",
                            Operator::Xor => "eor",
                            _ => "add",
                        };
                        self.stmts.push(format!("ldr r2, ={}", OFFSET + e1_val));
                        self.stmts.push(format!("ldr r3, ={}", OFFSET + e2_val));
                        self.stmts.push(format!("{} r4, r2, r3", instruction_name,));
                        self.stmts
                            .push(format!("str r4, ={}", OFFSET + curr_mem_loc));
                    }
                };
            }
            Expr::Unary(ref op, ref e1) => match op {
                Operator::PreIncr | Operator::PreDecr => {
                    if let Expr::Identifier(ref name) = e1.inside {
                        if let Some(var) = get_var(name.to_string(), &self.symbol_tables) {
                            self.stmts
                                .push(format!("ldr r2, ={}", OFFSET + var.mem_loc));

                            if let Operator::PreIncr = op {
                                self.stmts.push(format!("add r2, r2, #1"));
                            } else if let Operator::PreDecr = op {
                                self.stmts.push(format!("sub r2, r2, #1"));
                            }
                            self.stmts
                                .push(format!("str r2, ={}", OFFSET + var.mem_loc));

                            return_mem = var.mem_loc;
                        } else {
                            let pos = get_pos(&self.source_code, expr.byte_offset);
                            return Err(CompilationError::new(format!(
                                "Variable not found at line {}",
                                pos.0,
                            )));
                        }
                    } else {
                        let line = get_pos(&self.source_code, expr.byte_offset).0;
                        return Err(CompilationError::new(format!(
                            "{:?} can only be used on a variable, error at line {}",
                            op, line
                        )));
                    }
                }
                Operator::PostIncr | Operator::PostDecr => {
                    if let Expr::Identifier(ref name) = e1.inside {
                        if let Some(var) = get_var(name.to_string(), &self.symbol_tables) {
                            self.stmts
                                .push(format!("ldr r2, ={}", OFFSET + var.mem_loc));
                            self.stmts
                                .push(format!("str r2, ={}", OFFSET + curr_mem_loc));
                            if let Operator::PostIncr = op {
                                self.stmts.push(format!("add r2, r2, #1"));
                            } else if let Operator::PostDecr = op {
                                self.stmts.push(format!("sub r2, r2, #1"));
                            }
                            self.stmts
                                .push(format!("str r2, ={}", OFFSET + var.mem_loc));
                            return_mem = curr_mem_loc;
                        } else {
                            let pos = get_pos(&self.source_code, expr.byte_offset);
                            return Err(CompilationError::new(format!(
                                "Variable not found at line {}",
                                pos.0,
                            )));
                        }
                    } else {
                        let line = get_pos(&self.source_code, expr.byte_offset).0;
                        return Err(CompilationError::new(format!(
                            "{:?} can only be used on a variable, error at line {}",
                            op, line
                        )));
                    }
                }
                Operator::Negate => {
                    let e1_val = self.compile_expr(&*e1)?;
                    self.stmts.push(format!("ldr r2, ={}", OFFSET + e1_val));
                    self.stmts.push(format!("rsb r2, r2, #0"));
                    self.stmts
                        .push(format!("str r2, ={}", OFFSET + curr_mem_loc));
                }
                Operator::Positive => {
                    let e1_val = self.compile_expr(&*e1)?;
                    return_mem = e1_val;
                }
                Operator::Not => {
                    let e1_val = self.compile_expr(&*e1)?;
                    self.stmts.push(format!("ldr r2, ={}", OFFSET + e1_val));
                    self.stmts.push(format!("cmp r2, #0"));
                    self.stmts.push(format!("moveq r3, #1"));
                    self.stmts.push(format!("movne r3, #0"));
                    self.stmts
                        .push(format!("str r3, ={}", OFFSET + curr_mem_loc));
                }
                _ => (),
            },
            Expr::FunctionCall(ref name, ref args) => {
                self.count += 4;
                let return_val_loc = self.count;

                self.count += 4;
                let return_loc = self.count;

                let mut expr_vals = vec![];
                for arg in args {
                    expr_vals.push(self.compile_expr(&*arg)?);
                }

                let function_def = get_var(name.clone(), &self.symbol_tables).unwrap().clone();
                let param_locs = function_def.param_locs.unwrap();

                if let Some(ref mut table_for_scope) = self.symbol_tables.last_mut() {
                    table_for_scope.insert(
                        CURRENT_FUNCTION_CALL.to_string(),
                        SymbolEntry::new(
                            &mut self.count,
                            Some(vec![return_loc, return_val_loc]),
                            None,
                        ),
                    );
                }
                let label = format!("function{}", name);
                for (i, expr_val) in expr_vals.iter().enumerate() {
                    if let Some(param_loc) = param_locs.get(i) {
                        self.stmts.push(format!("ldr r1, ={}", OFFSET + expr_val));
                        self.stmts.push(format!("str r1, ={}", OFFSET + param_loc));
                    }
                }
                self.stmts.push(format!("ldr pc, ={}", OFFSET + return_loc));
                self.stmts.push(format!("bl {}", label));
                if let Some(return_val_loc) = param_locs.last() {
                    return_mem = *return_val_loc;
                }
            }
            Expr::Assignment(ref lhs, ref rhs) => {
                let rhs_val = self.compile_expr(&*rhs)?;
                if let Expr::Identifier(ref name) = lhs.inside {
                    if let Some(var) = get_var(name.to_string(), &self.symbol_tables) {
                        self.stmts.push(format!("ldr r3, ={}", OFFSET + rhs_val));
                        self.stmts
                            .push(format!("str r3, ={}", OFFSET + var.mem_loc));
                        return_mem = var.mem_loc;
                    } else {
                        let pos = get_pos(&self.source_code, expr.byte_offset);
                        return Err(CompilationError::new(format!(
                            "Assigning to undeclared variable {} at line: {}",
                            name, pos.0,
                        )));
                    }
                } else {
                    let line = get_pos(&self.source_code, expr.byte_offset).0;
                    return Err(CompilationError::new(format!(
                        "Can only assign to a variable, error at line {}",
                        line
                    )));
                }
            }
            Expr::Identifier(ref name) => {
                if let Some(var) = get_var(name.to_string(), &self.symbol_tables) {
                    return_mem = var.mem_loc;
                } else {
                    let pos = get_pos(&self.source_code, expr.byte_offset);
                    return Err(CompilationError::new(format!(
                        "Using undeclared variable {} at line {}",
                        name, pos.0,
                    )));
                }
            }
            Expr::Number(num) => {
                self.stmts
                    .push(format!(".equ const{}, {}", curr_mem_loc, num));
                self.stmts.push(format!("ldr r3, =const{}", curr_mem_loc));
                self.stmts
                    .push(format!("str r3, ={}", OFFSET + curr_mem_loc));
            }
            _ => (),
        };

        Ok(return_mem)
    }
}

fn get_var<'a>(var_name: String, symbol_tables: &'a SymbolTables) -> Option<&'a SymbolEntry> {
    for table in symbol_tables.iter().rev() {
        if let Some(symbol_entry) = table.get(&var_name) {
            return Some(symbol_entry);
        }
    }
    None
}

fn get_function_def<'a>(symbol_tables: &'a SymbolTables) -> Option<&'a SymbolEntry> {
    for table in symbol_tables.iter().rev() {
        return table.get(CURRENT_FUNCTION_CALL);
    }
    None
}
