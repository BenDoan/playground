#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod ast;
pub mod compiler;
pub mod util;

use std::io;
use std::io::BufRead;
use lalrpop_util::ParseError;
use ast::Program;
use util::get_pos;

lalrpop_mod!(pub parser);

fn main() {
    let stdin = io::stdin();
    let program_string = stdin
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(program) = parse_program(&program_string) {
        // let program = add_line_nums(&program, &program_string).clone();
        handle_ast(&program, &program_string);
    }
}

fn handle_ast(ast: &Program, source_code: &String) {
    match compiler::Compiler::new(source_code.clone()).compile(ast) {
        Ok(stmts) => println!("{}", stmts.join("\n")),
        Err(err) => println!("Compilation Error: {}", err),
    }
}

fn parse_program(program_string: &String) -> Option<Program> {
    let maybe_ast = parser::ProgramParser::new().parse(program_string);
    match maybe_ast {
        Ok(ast) => Some(ast),
        Err(e) => {
            match e {
                ParseError::UnrecognizedToken { token, .. } => {
                    if let Some((byte, ..)) = token {
                        let (line, _) = get_pos(program_string, byte);
                        println!("Syntax Error at line: {}", line);
                    }
                }
                ParseError::InvalidToken { location } => {
                    let (line, _) = get_pos(program_string, location);
                    println!("Invalid token at line {}", line);
                }
                misc @ _ => println!("{:?}", misc),
            }
            None
        }
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
