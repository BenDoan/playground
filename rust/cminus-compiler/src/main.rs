#[macro_use]
extern crate lalrpop_util;

pub mod ast;

use std::io;
use std::io::BufRead;
use ast::{Program, Parameter, Stmt, Expr};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

lalrpop_mod!(pub parser);

fn main() {
    let stdin = io::stdin();
    let program_string = stdin
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .collect::<String>();

    let ast = parser::ProgramParser::new().parse(&program_string).unwrap();
    println!("Default AST format:\n{:?}\n\n", ast);
    println!("Traversal AST format:\n{}\n\n", ast_to_str(&ast));
    println!("Processing symbol table:");
    process_symbol_table(&ast);
}

struct SymbolEntry {
    param: Parameter,
}

type SymbolTables = Rc<RefCell<Vec<Rc<RefCell<HashMap<String, SymbolEntry>>>>>>;

fn process_symbol_table(program: &Program) {
    fn traverse_stmt(stmt: &Stmt, symbol_tables: SymbolTables) {
        match stmt {
            Stmt::Block(stmts) => {
                let table_for_scope = Rc::new(RefCell::new(HashMap::new()));
                symbol_tables.borrow_mut().push(table_for_scope.clone());
                for stmt in stmts {
                    traverse_stmt(stmt, symbol_tables.clone());
                }
                for (var_name, _) in table_for_scope.borrow_mut().iter() {
                    println!("Variable {} out of scope", var_name);
                }
                symbol_tables.borrow_mut().pop();
            }
            Stmt::Declaration(parameters) => {
                for parameter in parameters {
                    println!("New variable {:?} in scope", parameter.identifier);
                    if let Some(table_for_scope) = symbol_tables.borrow_mut().last() {
                        table_for_scope.borrow_mut().insert(
                            parameter.identifier.clone(),
                            SymbolEntry { param: parameter.clone() },
                        );
                    }
                }
            }
            Stmt::Function(_, parameters, statement) => {
                for parameter in parameters {
                    println!("New param {:?} in scope", parameter.identifier);
                    if let Some(table_for_scope) = symbol_tables.borrow_mut().last() {
                        table_for_scope.borrow_mut().insert(
                            parameter.identifier.clone(),
                            SymbolEntry { param: parameter.clone() },
                        );
                    }
                }
                traverse_stmt(statement, symbol_tables);
            }
            Stmt::While(_, statement) => traverse_stmt(statement, symbol_tables),
            Stmt::For(_, _, _, statement) => traverse_stmt(statement, symbol_tables),
            _ => (),
        }
    }

    let symbol_tables: SymbolTables = Rc::new(RefCell::new(vec![]));
    for statement in program {
        traverse_stmt(statement, symbol_tables.clone());
    }
}

fn ast_to_str(program: &Program) -> String {
    fn traverse_stmt(stmt: &Stmt, level: usize) -> String {
        let i = get_indent(level);
        match stmt {
            Stmt::Block(ss) => {
                ss.iter()
                    .map(|s| traverse_stmt(s, level + 1))
                    .collect::<Vec<String>>()
                    .join("")
            }
            Stmt::If(e, s) => {
                format!(
                    "{}IF {}:\n{}\n",
                    i,
                    traverse_expr(e),
                    traverse_stmt(s, level + 1)
                )
            }
            Stmt::While(e, s) => {
                format!(
                    "{}WHILE {}:\n{}\n",
                    i,
                    traverse_expr(e),
                    traverse_stmt(s, level + 1)
                )
            }
            Stmt::For(e1, e2, e3, s) => {
                format!(
                    "{}FOR {}, {}, {}:\n{}\n",
                    i,
                    traverse_expr(e1),
                    traverse_expr(e2),
                    traverse_expr(e3),
                    traverse_stmt(s, level + 1)
                )
            }
            Stmt::Return(e) => format!("{}RETURN {}\n", i, traverse_expr(e)),
            Stmt::Read(s) => format!("{}READ {}\n", i, s),
            Stmt::Write(e) => format!("{}WRITE {}\n", i, traverse_expr(e)),
            Stmt::Expr(e) => format!("{}{}\n", i, traverse_expr(e)),
            Stmt::Function(f, ps, s) => {
                format!(
                    "{}FN {} ({}):\n{}\n",
                    i,
                    f,
                    get_params(ps),
                    traverse_stmt(s, level + 1)
                )
            }
            Stmt::Declaration(ps) => format!("{}DECL {}\n", i, get_params(ps)),
        }
    }

    fn traverse_expr(expr: &Expr) -> String {
        match expr {
            Expr::Binary(o, e1, e2) => {
                format!("{} {:?} {}", traverse_expr(e1), o, traverse_expr(e2))
            }
            Expr::Unary(o, e) => format!("{:?} {}", o, traverse_expr(e)),
            Expr::Number(n) => n.to_string(),
            Expr::Str(s) => s.to_string(),
            Expr::Identifier(s) => s.to_string(),
            Expr::Assignment(e1, e2) => {
                format!("int {} = {}", traverse_expr(e1), traverse_expr(e2))
            }
            Expr::FunctionCall(s, es) => {
                format!(
                    "{}({})",
                    s,
                    es.iter().map(traverse_expr).collect::<Vec<String>>().join(
                        ", ",
                    )
                )
            }
        }
    }
    fn get_params(parameters: &Vec<Parameter>) -> String {
        parameters
            .iter()
            .map(|p| format!("int {}", p.identifier))
            .collect::<Vec<String>>()
            .join(", ")
    }


    program.iter().map(|s| traverse_stmt(s, 0)).collect()
}

fn get_indent(level: usize) -> String {
    "  ".repeat(level).to_string()
}
