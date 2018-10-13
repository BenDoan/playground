#[macro_use]
extern crate lalrpop_util;

pub mod ast;

use std::io;
use std::io::BufRead;
use ast::{Meta, Program, Parameter, Stmt, Expr, Operator};
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
    println!("Processing symbol table:");
    process_symbol_table(&ast);
}

struct SymbolEntry {
    param: Parameter,
}

type SymbolTables = Vec<HashMap<String, SymbolEntry>>;

fn process_symbol_table(program: &Program) {
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
                            "Variable {} out of scope for line {}",
                            var_name,
                            stmt.byte_offset
                        );
                    }
                }
            }
            Stmt::Declaration(ref parameters) => {
                for parameter in parameters {
                    println!(
                        "New variable {:?} in scope for line {}",
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
                        "New param {:?} in scope at line {}",
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
