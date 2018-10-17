use ast::{Meta, Program, Parameter, Stmt, Expr, Operator};
use std::collections::HashMap;
use util::get_pos;

type SymbolTables = Vec<HashMap<String, SymbolEntry>>;

pub struct SymbolEntry {
    param: Parameter,
}

pub fn compile(program: &Program, source_code: &String) {
    fn traverse(stmt: &Meta<Stmt>, symbol_tables: &mut SymbolTables, source_code: &String) {
        println!("\n{:?}", (
            "comment",
            format!(
                "line {}",
                get_pos(source_code, stmt.byte_offset).0
            ),
        ));

        match stmt.inside {
            Stmt::Block(ref stmts) => {
                symbol_tables.push(HashMap::new());
                for stmt in stmts {
                    traverse(&stmt, symbol_tables, source_code);
                }

                if let Some(table_for_scope) = symbol_tables.pop() {
                    for (var_name, _) in table_for_scope.iter() {
                        println!(
                            "variable {} out of scope for line {}",
                            var_name,
                            stmt.byte_offset
                        );
                    }
                }
            }
            Stmt::Declaration(ref parameters) => {
                for parameter in parameters {
                    println!(
                        "new variable {:?} in scope for line {}",
                        parameter.identifier,
                        stmt.get_line()
                    );
                    if let Some(table_for_scope) = symbol_tables.last_mut() {
                        table_for_scope.insert(
                            parameter.identifier.clone(),
                            SymbolEntry { param: parameter.clone() },
                        );
                    }
                }
            }
            Stmt::Function(ref _name, ref parameters, ref statement) => {
                for parameter in parameters {
                    println!(
                        "new param {:?} in scope at line {}",
                        parameter.identifier,
                        stmt.byte_offset
                    );
                    if let Some(table_for_scope) = symbol_tables.last_mut() {
                        table_for_scope.insert(
                            parameter.identifier.clone(),
                            SymbolEntry { param: parameter.clone() },
                        );
                    }
                }
                traverse(&*statement, symbol_tables, source_code);
            }
            Stmt::While(_, ref statement) => traverse(&*statement, symbol_tables, source_code),
            Stmt::For(_, _, _, ref statement) => traverse(&*statement, symbol_tables, source_code),
            Stmt::Expr(ref expr) => {
                compile_expr(&expr, symbol_tables);
            }
            _ => {
                println!("{:?}", ("unimplemented",));
            }
        }
    }

    let mut symbol_tables: SymbolTables = vec![];
    for statement in program {
        traverse(statement, &mut symbol_tables, source_code);
    }
}

pub fn compile_expr(expr: &Meta<Expr>, symbol_tables: &mut SymbolTables) -> usize {
    match expr.inside {
        Expr::Binary(ref op, ref e1, ref e2) => {
            match op {
                Operator::LogicalAnd => {
                    let e1_loc = compile_expr(&*e1, symbol_tables);
                    println!(
                        "{:?}",
                        ("test", e1_loc, "if false jump to", expr.byte_offset)
                    );
                    let e2_loc = compile_expr(&*e2, symbol_tables);
                    println!("{:?}", ("label", expr.byte_offset));
                    println!("{:?}", ("and", e1_loc, e2_loc, expr.byte_offset));
                }
                Operator::LogicalOr => {
                    let e1_loc = compile_expr(&*e1, symbol_tables);
                    println!(
                        "{:?}",
                        ("test", e1_loc, "if true jump to", expr.byte_offset)
                    );
                    let e2_loc = compile_expr(&*e2, symbol_tables);
                    println!("{:?}", ("label", expr.byte_offset));
                    println!("{:?}", ("and", e1_loc, e2_loc, expr.byte_offset));
                }
                _ => {
                    let e1_val = compile_expr(&*e1, symbol_tables);
                    let e2_val = compile_expr(&*e2, symbol_tables);
                    println!("{:?}", (
                        "binary-math-op",
                        op,
                        e1_val,
                        e2_val,
                        expr.byte_offset,
                    ));
                }
            };
            expr.byte_offset
        }
        Expr::Unary(ref op, ref e1) => {
            let e1_val = compile_expr(&*e1, symbol_tables);
            println!("{:?}", ("unary-math-op", op, e1_val, expr.byte_offset));
            expr.byte_offset

        }
        Expr::Assignment(ref lhs, ref rhs) => {
            let _var_name = match lhs.inside {
                Expr::Identifier(ref s) => s.to_string(),
                _ => "".to_string(),
            };
            let rhs_val = compile_expr(&*rhs, symbol_tables);
            println!("{:?}", ("store", rhs_val, lhs.byte_offset));
            lhs.byte_offset
        }
        Expr::Identifier(ref name) => {
            println!("{:?}", ("load", expr.byte_offset, name));
            expr.byte_offset
        }
        Expr::Number(num) => {
            println!("{:?}", ("word", expr.byte_offset, num));
            expr.byte_offset
        }
        _ => 0,
    }
}
