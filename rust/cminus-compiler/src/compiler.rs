use ast::{Meta, Program, Parameter, Stmt, Expr, Operator};
use std::collections::HashMap;
use util::get_pos;

pub fn compile(program: &Program, source_code: &String) {
    fn traverse(stmt: &Meta<Stmt>, source_code: &String) {
        println!("{:?}", (
            "comment",
            format!(
                "line {}",
                get_pos(source_code, stmt.byte_offset).0
            ),
        ));

        match stmt.inside {
            Stmt::Block(ref stmts) => {
                for stmt in stmts {
                    traverse(&stmt, source_code);
                }
            }
            Stmt::Function(_, _, ref stmt) => {
                traverse(&*stmt, source_code);
            }
            Stmt::While(_, ref statement) => traverse(&*statement, source_code),
            Stmt::For(_, _, _, ref statement) => traverse(&*statement, source_code),
            Stmt::Expr(ref expr) => {
                compile_expr(&expr);
            }
            _ => (),
        }
    }

    for statement in program {
        traverse(statement, source_code);
    }
}

pub fn compile_expr(expr: &Meta<Expr>) -> usize {
    match expr.inside {
        Expr::Binary(ref op, ref e1, ref e2) => {
            let e1_val = compile_expr(&*e1);
            let e2_val = compile_expr(&*e2);
            println!("{:?}", (
                "binary-math-op",
                op,
                e1_val,
                e2_val,
                expr.byte_offset,
            ));
            expr.byte_offset

        }
        Expr::Unary(ref op, ref e1) => {
            let e1_val = compile_expr(&*e1);
            println!("{:?}", ("unary-math-op", op, e1_val));
            expr.byte_offset

        }
        Expr::Assignment(ref lhs, ref rhs) => {
            let lhs_val = compile_expr(&*lhs);
            let rhs_val = compile_expr(&*rhs);
            rhs_val
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

struct SymbolEntry {
    param: Parameter,
}

type SymbolTables = Vec<HashMap<String, SymbolEntry>>;

pub fn process_symbol_table(program: &Program) {
    fn traverse_stmt(stmt: Meta<Stmt>, symbol_tables: &mut SymbolTables) {
        match stmt.inside {
            Stmt::Block(stmts) => {
                symbol_tables.push(HashMap::new());
                for stmt in stmts {
                    traverse_stmt(stmt, symbol_tables);
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
            Stmt::Function(_, parameters, statement) => {
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
                traverse_stmt(*statement, symbol_tables);
            }
            Stmt::While(_, statement) => traverse_stmt(*statement, symbol_tables),
            Stmt::For(_, _, _, statement) => traverse_stmt(*statement, symbol_tables),
            _ => (),
        }
    }

    let mut symbol_tables: SymbolTables = vec![];
    for statement in program {
        traverse_stmt(statement.clone(), &mut symbol_tables);
    }
}
