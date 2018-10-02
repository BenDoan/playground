#[macro_use]
extern crate lalrpop_util;

pub mod ast;

use std::io;
use std::io::BufRead;
use ast::{Program, Parameter, Stmt, Expr, Operator};
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
                    .collect::<Vec<_>>()
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
                    es.iter().map(traverse_expr).collect::<Vec<_>>().join(", ")
                )
            }
        }
    }
    fn get_params(parameters: &Vec<Parameter>) -> String {
        parameters
            .iter()
            .map(|p| format!("int {}", p.identifier))
            .collect::<Vec<_>>()
            .join(", ")
    }


    program.iter().map(|s| traverse_stmt(s, 0)).collect()
}

fn get_indent(level: usize) -> String {
    "  ".repeat(level).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        assert_eq!(
            parser::ExprParser::new().parse("1").unwrap(),
            Expr::Number(1)
        );
    }

    #[test]
    fn test_string() {
        assert_eq!(
            parser::ExprParser::new().parse("\"test\"").unwrap(),
            Expr::Str("test".to_string())
        );
    }

    #[test]
    fn test_identifier() {
        assert_eq!(
            parser::ExprParser::new().parse("test").unwrap(),
            Expr::Identifier("test".to_string())
        );
    }

    #[test]
    fn test_binary_expr() {
        assert_eq!(
            parser::ExprParser::new().parse("1 + 2").unwrap(),
            Expr::Binary(
                Operator::Add,
                Box::new(Expr::Number(1)),
                Box::new(Expr::Number(2)),
            )
        );
    }

    #[test]
    fn test_unary_expr() {
        assert_eq!(
            parser::ExprParser::new().parse("1++").unwrap(),
            Expr::Unary(Operator::PostIncr, Box::new(Expr::Number(1)))
        );
    }

    #[test]
    fn test_block() {
        assert_eq!(
            parser::StatementParser::new().parse("{ }").unwrap(),
            Stmt::Block(vec![])
        );

        assert_eq!(
            parser::StatementParser::new().parse("{ 1; 2; }").unwrap(),
            Stmt::Block(vec![
                Stmt::Expr(Expr::Number(1)),
                Stmt::Expr(Expr::Number(2)),
            ])
        );
    }

    #[test]
    fn test_if() {
        assert_eq!(
            parser::StatementParser::new().parse("if (1) {}").unwrap(),
            Stmt::If(Expr::Number(1), Box::new(Stmt::Block(vec![])))
        );
    }

    #[test]
    fn test_while() {
        assert_eq!(
            parser::StatementParser::new()
                .parse("while (1) {}")
                .unwrap(),
            Stmt::While(Expr::Number(1), Box::new(Stmt::Block(vec![])))
        );
    }

    #[test]
    fn test_for() {
        assert_eq!(
            parser::StatementParser::new()
                .parse("for(i = 1; i < 1; i++) {} ")
                .unwrap(),
            Stmt::For(
                Expr::Assignment(
                    Box::new(Expr::Identifier("i".to_string())),
                    Box::new(Expr::Number(1)),
                ),
                Expr::Binary(
                    Operator::Less,
                    Box::new(Expr::Identifier("i".to_string())),
                    Box::new(Expr::Number(1)),
                ),
                Expr::Unary(
                    Operator::PostIncr,
                    Box::new(Expr::Identifier("i".to_string())),
                ),
                Box::new(Stmt::Block(vec![])),
            )
        );
    }

    #[test]
    fn test_return() {
        assert_eq!(
            parser::StatementParser::new().parse("return 1;").unwrap(),
            Stmt::Return(Expr::Number(1))
        );
    }

    #[test]
    fn test_read() {
        assert_eq!(
            parser::StatementParser::new().parse("read(test);").unwrap(),
            Stmt::Read("test".to_string())
        );
    }

    #[test]
    fn test_write() {
        assert_eq!(
            parser::StatementParser::new().parse("write(1);").unwrap(),
            Stmt::Write(Expr::Number(1))
        );
    }

    #[test]
    fn test_expr_stmt() {
        assert_eq!(
            parser::StatementParser::new().parse("1;").unwrap(),
            Stmt::Expr(Expr::Number(1))
        );
    }

    #[test]
    fn test_assignment() {
        assert_eq!(
            parser::ExprParser::new().parse("i = 1").unwrap(),
            Expr::Assignment(
                Box::new(Expr::Identifier("i".to_string())),
                Box::new(Expr::Number(1)),
            )
        );
    }

    #[test]
    fn test_function_call() {
        assert_eq!(
            parser::StatementParser::new().parse("foo();").unwrap(),
            Stmt::Expr(Expr::FunctionCall("foo".to_string(), vec![]))
        );

        assert_eq!(
            parser::StatementParser::new().parse("foo(i, j);").unwrap(),
            Stmt::Expr(Expr::FunctionCall(
                "foo".to_string(),
                vec![
                    Expr::Identifier("i".to_string()),
                    Expr::Identifier("j".to_string()),
                ],
            ))
        );
    }

    #[test]
    fn test_declaration() {
        assert_eq!(
            parser::DeclarationParser::new()
                .parse("int i[1][2], j[3][4];")
                .unwrap(),
            Stmt::Declaration(vec![
                Parameter {
                    identifier: "i".to_string(),
                    sub_arrays: vec![1, 2],
                },
                Parameter {
                    identifier: "j".to_string(),
                    sub_arrays: vec![3, 4],
                },
            ])
        );
    }

    #[test]
    fn test_function_declaration() {
        assert_eq!(
            parser::ExternalDeclarationParser::new()
                .parse("int foo() {}")
                .unwrap(),
            Stmt::Function("foo".to_string(), vec![], Box::new(Stmt::Block(vec![])))
        );

        assert_eq!(
            parser::ExternalDeclarationParser::new()
                .parse("int foo(int i, int j[]) {}")
                .unwrap(),
            Stmt::Function(
                "foo".to_string(),
                vec![
                    Parameter {
                        identifier: "i".to_string(),
                        sub_arrays: vec![],
                    },
                    Parameter {
                        identifier: "j".to_string(),
                        sub_arrays: vec![-1],
                    },
                ],
                Box::new(Stmt::Block(vec![])),
            )
        );
    }

}
