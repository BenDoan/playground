use ast::{Meta, Program, Parameter, Stmt, Expr, Operator};
use std::collections::HashMap;
use util::get_pos;

const OFFSET: usize = 6010000;

type SymbolTables = Vec<HashMap<String, SymbolEntry>>;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SymbolEntry {
    id: usize,
    name: String,
    mem_loc: usize,
}

impl SymbolEntry {
    pub fn new(id: usize, name: String, mem_loc: &mut usize) -> SymbolEntry {
        *mem_loc += 4;
        SymbolEntry {
            id: id,
            name: name,
            mem_loc: *mem_loc,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Quad {
    Comment(String),
    Ident(usize),
    Constant(usize, i32),
    Binary(usize, Operator, usize, usize),
    Unary(usize, Operator, usize),
    Assign(usize, usize),
    Write(usize),
    Unimplemented,
}

pub fn compile(program: &Program, source_code: &String) -> Vec<Quad> {
    fn traverse(
        stmt: &Meta<Stmt>,
        symbol_tables: &mut SymbolTables,
        quads: &mut Vec<Quad>,
        source_code: &String,
        count: &mut usize,
    ) {
        quads.push(Quad::Comment(
            format!("line {}", get_pos(source_code, stmt.byte_offset).0),
        ));

        match stmt.inside {
            Stmt::Block(ref stmts) => {
                symbol_tables.push(HashMap::new());
                for stmt in stmts {
                    traverse(&stmt, symbol_tables, quads, source_code, count);
                }
                symbol_tables.pop();
            }
            Stmt::Declaration(ref parameters) => {
                for parameter in parameters {
                    if let Some(table_for_scope) = symbol_tables.last_mut() {
                        table_for_scope.insert(
                            parameter.identifier.clone(),
                            SymbolEntry::new(
                                stmt.byte_offset,
                                parameter.identifier.clone(),
                                count,
                            ),
                        );
                    }
                }
            }
            Stmt::Function(ref _name, ref parameters, ref statement) => {
                for parameter in parameters {
                    if let Some(table_for_scope) = symbol_tables.last_mut() {
                        table_for_scope.insert(
                            parameter.identifier.clone(),
                            SymbolEntry::new(
                                stmt.byte_offset,
                                parameter.identifier.clone(),
                                count,
                            ),
                        );
                    }
                }
                traverse(&*statement, symbol_tables, quads, source_code, count);
            }
            Stmt::Write(ref expr) => {
                let expr_val = compile_expr(&expr, symbol_tables, quads, count);
                quads.push(Quad::Write(expr_val));
            }
            Stmt::While(_, ref statement) => {
                traverse(&*statement, symbol_tables, quads, source_code, count)
            }
            Stmt::For(_, _, _, ref statement) => {
                traverse(&*statement, symbol_tables, quads, source_code, count)
            }
            Stmt::Expr(ref expr) => {
                compile_expr(&expr, symbol_tables, quads, count);
            }
            _ => {
                quads.push(Quad::Unimplemented);
            }
        }
    }

    let mut symbol_tables: SymbolTables = vec![HashMap::new()];
    let mut quads = vec![];
    let mut count = 0;
    for statement in program {
        traverse(
            statement,
            &mut symbol_tables,
            &mut quads,
            source_code,
            &mut count,
        );
    }

    println!("; Symbol table:");
    for symbol_table in symbol_tables {
        for (k, symbol) in &symbol_table {
            println!("; {}={:?}", k, symbol);
        }
    }

    quads.clone()
}

pub fn compile_expr(
    expr: &Meta<Expr>,
    symbol_tables: &mut SymbolTables,
    quads: &mut Vec<Quad>,
    count: &mut usize,
) -> usize {
    let curr_mem_loc = *count + 4;
    let mut return_mem = curr_mem_loc;
    let mut val = "".to_string();
    match expr.inside {
        Expr::Binary(ref op, ref e1, ref e2) => {
            val = format!("{:?}", op);
            match op {
                Operator::LogicalAnd => {
                    let e1_loc = compile_expr(&*e1, symbol_tables, quads, count);
                    // println!(
                    //     "{:?}",
                    //     ("test", e1_loc, "if false jump to", expr.byte_offset)
                    // );
                    let e2_loc = compile_expr(&*e2, symbol_tables, quads, count);
                    // println!("{:?}", ("label", expr.byte_offset));
                    // println!("{:?}", ("and", e1_loc, e2_loc, expr.byte_offset));
                }
                Operator::LogicalOr => {
                    let e1_loc = compile_expr(&*e1, symbol_tables, quads, count);
                    // println!(
                    //     "{:?}",
                    //     ("test", e1_loc, "if true jump to", expr.byte_offset)
                    // );
                    let e2_loc = compile_expr(&*e2, symbol_tables, quads, count);
                    // println!("{:?}", ("label", expr.byte_offset));
                    // println!("{:?}", ("and", e1_loc, e2_loc, expr.byte_offset));
                }
                _ => {
                    let e1_val = compile_expr(&*e1, symbol_tables, quads, count);
                    let e2_val = compile_expr(&*e2, symbol_tables, quads, count);
                    quads.push(Quad::Binary(curr_mem_loc, *op, e1_val, e2_val));
                }
            };
        }
        Expr::Unary(ref op, ref e1) => {
            val = format!("{:?}", op);
            match op {
                Operator::PreIncr | Operator::PreDecr => {
                    let mem_loc = match e1.inside {
                        Expr::Identifier(ref name) => {
                            val = name.to_string();
                            symbol_tables.first().unwrap().get(name).unwrap().mem_loc
                        }
                        _ => 0,
                    };
                    quads.push(Quad::Unary(curr_mem_loc, *op, mem_loc));
                    return_mem = mem_loc;
                }
                Operator::PostIncr | Operator::PostDecr => {
                    let mem_loc = match e1.inside {
                        Expr::Identifier(ref name) => {
                            val = name.to_string();
                            symbol_tables.first().unwrap().get(name).unwrap().mem_loc
                        }
                        _ => 0,
                    };
                    quads.push(Quad::Unary(curr_mem_loc, *op, mem_loc));
                }
                _ => {
                    let e1_val = compile_expr(&*e1, symbol_tables, quads, count);
                    quads.push(Quad::Unary(curr_mem_loc, *op, e1_val));
                }
            }
        }
        Expr::Assignment(ref lhs, ref rhs) => {
            let mem_loc = match lhs.inside {
                Expr::Identifier(ref name) => {
                    val = name.to_string();
                    symbol_tables.first().unwrap().get(name).unwrap().mem_loc
                }
                _ => 0,
            };
            let rhs_val = compile_expr(&*rhs, symbol_tables, quads, count);
            quads.push(Quad::Assign(mem_loc, rhs_val));
        }
        Expr::Identifier(ref name) => {
            // println!("{:?}", ("load", expr.byte_offset, name));
            val = name.to_string();
            if let Some(symbol_table) = symbol_tables.first_mut() {
                if let Some(symbol) = symbol_table.get(name) {
                    return_mem = symbol.mem_loc;
                    quads.push(Quad::Ident(symbol.mem_loc));
                }
            }
        }
        Expr::Number(num) => {
            val = num.to_string();
            quads.push(Quad::Constant(curr_mem_loc, num));
        }
        _ => (),
    };

    let curr_symbol = SymbolEntry::new(expr.byte_offset, val, count);

    if let Some(symbol_table) = symbol_tables.first_mut() {
        symbol_table.insert(expr.byte_offset.to_string(), curr_symbol.clone());
    }
    return_mem
}

pub fn quads_to_arm(quads: &Vec<Quad>) -> Vec<String> {
    let mut stmts = vec![];
    for quad in quads.iter() {
        match quad {
            Quad::Comment(comment) => {
                stmts.push("".to_string());
                stmts.push(format!("; {}", comment));
            }
            Quad::Constant(loc, val) => {
                stmts.push(format!(".equ const{}, {}", loc, val));
                stmts.push(format!("ldr r3, =const{}", loc));
                stmts.push(format!("str r3, ={}", OFFSET + loc));
            }
            Quad::Assign(loc, val) => {
                stmts.push(format!("ldr r3, ={}", OFFSET + val));
                stmts.push(format!("str r3, ={}", OFFSET + loc));
            }
            Quad::Binary(loc, op, val1, val2) => {
                match op {
                    Operator::Mod => {}
                    Operator::Divide => {}
                    _ => {
                        let instruction_name = match op {
                            Operator::Add => "add",
                            Operator::Subtract => "sub",
                            Operator::Multiply => "mul",
                            Operator::And => "and",
                            Operator::Or => "orr",
                            Operator::Xor => "eor",
                            _ => "add",
                        };
                        stmts.push(format!("ldr r2, ={}", OFFSET + val1));
                        stmts.push(format!("ldr r3, ={}", OFFSET + val2));
                        stmts.push(format!(
                            "{} r4, r2, r3",
                            instruction_name,
                        ));
                        stmts.push(format!("str r4, ={}", OFFSET + loc));
                    }
                }
            }
            Quad::Unary(loc, op, val) => {
                match op {
                    Operator::Negate => {
                        stmts.push(format!("ldr r2, ={}", OFFSET + val));
                        stmts.push(format!("rsb r2, r2, #0"));
                        stmts.push(format!("str r2, ={}", OFFSET + loc));
                    }
                    Operator::Not => {
                        stmts.push(format!("ldr r2, ={}", OFFSET + val));
                        stmts.push(format!("cmp r2, #0"));
                        stmts.push(format!("moveq r3, #1"));
                        stmts.push(format!("movne r3, #0"));
                        stmts.push(format!("str r3, ={}", OFFSET + loc));
                    }
                    Operator::PreIncr => {
                        stmts.push(format!("ldr r2, ={}", OFFSET + val));
                        stmts.push(format!("add r2, r2, #1"));
                        stmts.push(format!("str r2, ={}", OFFSET + loc));
                    }
                    _ => {
                        // let instruction_name = match op {
                        //     _ => "neg",
                        // };
                        // stmts.push(format!("ldr r2, ={}", OFFSET + val));
                        // stmts.push(format!(
                        //     "{} r3, r2",
                        //     instruction_name,
                        // ));
                        // stmts.push(format!("str r3, ={}", OFFSET + loc));
                    }
                };
            }
            Quad::Write(val) => {
                stmts.push(format!("mov r0, #1")); // print to stdout
                stmts.push(format!("ldr r1, ={}", OFFSET + val));
                stmts.push(format!("swi 0x6b")); // print val in r1
            }
            _ => (),
        }
    }
    stmts.push(format!("swi 0x11"));
    stmts
}
