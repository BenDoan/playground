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

pub fn compile(program: &Program, source_code: &String) -> Vec<String> {
    fn traverse(
        stmt: &Meta<Stmt>,
        symbol_tables: &mut SymbolTables,
        stmts: &mut Vec<String>,
        source_code: &String,
        count: &mut usize,
    ) {
        stmts.push(format!(
            "\n; line {}",
            get_pos(source_code, stmt.byte_offset).0
        ));

        match stmt.inside {
            Stmt::Block(ref statements) => {
                symbol_tables.push(HashMap::new());
                for stmt in statements {
                    traverse(&stmt, symbol_tables, stmts, source_code, count);
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
                traverse(&*statement, symbol_tables, stmts, source_code, count);
            }
            Stmt::Write(ref expr) => {
                let expr_val = compile_expr(&expr, symbol_tables, stmts, count);
                stmts.push(format!("mov r0, #1")); // configure for stdout
                stmts.push(format!("ldr r1, ={}", OFFSET + expr_val));
                stmts.push(format!("swi 0x6b")); // print val in r1
            }
            Stmt::While(_, ref statement) => {
                traverse(&*statement, symbol_tables, stmts, source_code, count)
            }
            Stmt::For(_, _, _, ref statement) => {
                traverse(&*statement, symbol_tables, stmts, source_code, count)
            }
            Stmt::Expr(ref expr) => {
                compile_expr(&expr, symbol_tables, stmts, count);
            }
            _ => (),

        }
    }

    let mut symbol_tables: SymbolTables = vec![HashMap::new()];
    let mut stmts = vec![];
    let mut count = 0;
    for statement in program {
        traverse(
            statement,
            &mut symbol_tables,
            &mut stmts,
            source_code,
            &mut count,
        );
    }

    stmts.push(format!("swi 0x11"));

    println!("; Symbol table:");
    for symbol_table in symbol_tables {
        for (k, symbol) in &symbol_table {
            println!("; {}={:?}", k, symbol);
        }
    }


    stmts.clone()
}

pub fn compile_expr(
    expr: &Meta<Expr>,
    symbol_tables: &mut SymbolTables,
    stmts: &mut Vec<String>,
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
                    let label1 = format!(".And1{}", curr_mem_loc);
                    let label2 = format!(".And2{}", curr_mem_loc);
                    let e1_loc = compile_expr(&*e1, symbol_tables, stmts, count);
                    stmts.push(format!("ldr r2, ={}", OFFSET + e1_loc));
                    stmts.push(format!("cmp r2, #0"));
                    stmts.push(format!("beq {}", label1));

                    let e2_loc = compile_expr(&*e2, symbol_tables, stmts, count);
                    stmts.push(format!("ldr r3, ={}", OFFSET + e2_loc));

                    stmts.push(format!("cmp r3, #0"));
                    stmts.push(format!("beq {}", label1));
                    stmts.push(format!("mov r3, #1"));
                    stmts.push(format!("b {}", label2));

                    stmts.push(format!("{}:", label1));
                    stmts.push(format!("mov r3, #0"));
                    stmts.push(format!("{}:", label2));
                    stmts.push(format!("str r3, ={}", OFFSET + curr_mem_loc));
                }
                Operator::LogicalOr => {
                    let label1 = format!(".Or1{}", curr_mem_loc);
                    let label2 = format!(".Or2{}", curr_mem_loc);
                    let label3 = format!(".Or3{}", curr_mem_loc);
                    let e1_loc = compile_expr(&*e1, symbol_tables, stmts, count);
                    stmts.push(format!("ldr r2, ={}", OFFSET + e1_loc));
                    stmts.push(format!("cmp r2, #0"));
                    stmts.push(format!("bne {}", label1));

                    let e2_loc = compile_expr(&*e2, symbol_tables, stmts, count);
                    stmts.push(format!("ldr r3, ={}", OFFSET + e2_loc));

                    stmts.push(format!("cmp r3, #0"));
                    stmts.push(format!("beq {}", label2));

                    stmts.push(format!("{}:", label1));
                    stmts.push(format!("mov r3, #1"));
                    stmts.push(format!("b {}", label3));

                    stmts.push(format!("{}:", label2));
                    stmts.push(format!("mov r3, #0"));

                    stmts.push(format!("{}:", label3));
                    stmts.push(format!("str r3, ={}", OFFSET + curr_mem_loc));
                }
                Operator::Greater | Operator::GreaterEqual | Operator::Less |
                Operator::LessEqual | Operator::Equal | Operator::NotEqual => {
                    let e1_loc = compile_expr(&*e1, symbol_tables, stmts, count);
                    let e2_loc = compile_expr(&*e2, symbol_tables, stmts, count);
                    stmts.push(format!("mov r1, #0"));
                    stmts.push(format!("ldr r2, ={}", OFFSET + e1_loc));
                    stmts.push(format!("ldr r3, ={}", OFFSET + e2_loc));
                    stmts.push(format!("cmp r2, r3"));
                    let instruction = match op {
                        Operator::Greater => "movgt",
                        Operator::GreaterEqual => "movge",
                        Operator::Less => "movlt",
                        Operator::LessEqual => "movle",
                        Operator::Equal => "moveq",
                        Operator::NotEqual => "movne",
                        _ => "",
                    };
                    stmts.push(format!("{} r1, #1", instruction));
                    stmts.push(format!("str r1, ={}", OFFSET + curr_mem_loc));
                }
                Operator::Divide | Operator::Mod => {
                    let label_start = format!(".start{}", curr_mem_loc);
                    let label_end = format!(".end{}", curr_mem_loc);

                    let e1_loc = compile_expr(&*e1, symbol_tables, stmts, count);
                    let e2_loc = compile_expr(&*e2, symbol_tables, stmts, count);
                    stmts.push(format!("ldr r1, ={}", OFFSET + e1_loc));
                    stmts.push(format!("ldr r2, ={}", OFFSET + e2_loc));
                    stmts.push(format!("mov r3, #0"));

                    stmts.push(format!("cmp r1, r2"));
                    stmts.push(format!("blt {}", label_end));

                    stmts.push(format!("{}:", label_start));
                    stmts.push(format!("sub r1, r1, r2"));
                    stmts.push(format!("add r3, r3, #1"));

                    stmts.push(format!("cmp r1, r2"));
                    stmts.push(format!("bge {}", label_start));
                    stmts.push(format!("{}:", label_end));
                    // r3 = quotient
                    // r1 = remainder

                    if let Operator::Divide = op {
                        stmts.push(format!("str r3, ={}", OFFSET + curr_mem_loc));
                    } else {
                        stmts.push(format!("str r1, ={}", OFFSET + curr_mem_loc));
                    }

                }
                _ => {
                    let e1_val = compile_expr(&*e1, symbol_tables, stmts, count);
                    let e2_val = compile_expr(&*e2, symbol_tables, stmts, count);
                    let instruction_name = match op {
                        Operator::Add => "add",
                        Operator::Subtract => "sub",
                        Operator::Multiply => "mul",
                        Operator::And => "and",
                        Operator::Or => "orr",
                        Operator::Xor => "eor",
                        _ => "add",
                    };
                    stmts.push(format!("ldr r2, ={}", OFFSET + e1_val));
                    stmts.push(format!("ldr r3, ={}", OFFSET + e2_val));
                    stmts.push(format!(
                        "{} r4, r2, r3",
                        instruction_name,
                    ));
                    stmts.push(format!("str r4, ={}", OFFSET + curr_mem_loc));
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
                    stmts.push(format!("ldr r2, ={}", OFFSET + mem_loc));

                    if let Operator::PreIncr = op {
                        stmts.push(format!("add r2, r2, #1"));
                    } else {
                        stmts.push(format!("sub r2, r2, #1"));
                    }
                    stmts.push(format!("ldr r2, ={}", OFFSET + mem_loc));

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
                    stmts.push(format!("ldr r2, ={}", OFFSET + mem_loc));
                    stmts.push(format!("str r2, ={}", OFFSET + curr_mem_loc));
                    if let Operator::PostIncr = op {
                        stmts.push(format!("add r2, r2, #1"));
                    } else {
                        stmts.push(format!("sub r2, r2, #1"));
                    }
                    stmts.push(format!("str r2, ={}", OFFSET + mem_loc));
                    return_mem = curr_mem_loc;
                }
                Operator::Negate => {
                    let e1_val = compile_expr(&*e1, symbol_tables, stmts, count);
                    stmts.push(format!("ldr r2, ={}", OFFSET + e1_val));
                    stmts.push(format!("rsb r2, r2, #0"));
                    stmts.push(format!("str r2, ={}", OFFSET + curr_mem_loc));
                }
                Operator::Positive => {
                    let e1_val = compile_expr(&*e1, symbol_tables, stmts, count);
                    return_mem = e1_val;
                }
                Operator::Not => {
                    let e1_val = compile_expr(&*e1, symbol_tables, stmts, count);
                    stmts.push(format!("ldr r2, ={}", OFFSET + e1_val));
                    stmts.push(format!("cmp r2, #0"));
                    stmts.push(format!("moveq r3, #1"));
                    stmts.push(format!("movne r3, #0"));
                    stmts.push(format!("str r3, ={}", OFFSET + curr_mem_loc));
                }
                _ => (),
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
            let rhs_val = compile_expr(&*rhs, symbol_tables, stmts, count);
            stmts.push(format!("ldr r3, ={}", OFFSET + rhs_val));
            stmts.push(format!("str r3, ={}", OFFSET + mem_loc));
        }
        Expr::Identifier(ref name) => {
            val = name.to_string();
            if let Some(symbol_table) = symbol_tables.first_mut() {
                if let Some(symbol) = symbol_table.get(name) {
                    return_mem = symbol.mem_loc;
                }
            }
        }
        Expr::Number(num) => {
            val = num.to_string();
            stmts.push(format!(".equ const{}, {}", curr_mem_loc, num));
            stmts.push(format!("ldr r3, =const{}", curr_mem_loc));
            stmts.push(format!("str r3, ={}", OFFSET + curr_mem_loc));
        }
        _ => (),
    };

    let curr_symbol = SymbolEntry::new(expr.byte_offset, val, count);
    if let Some(symbol_table) = symbol_tables.first_mut() {
        symbol_table.insert(expr.byte_offset.to_string(), curr_symbol.clone());
    }

    return_mem
}
